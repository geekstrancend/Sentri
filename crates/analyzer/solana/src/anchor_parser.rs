use crate::anchor_model::*;
use quote::ToTokens;
use syn::{Attribute, Field, Fields, Item, ItemStruct, Type};

/// Parse a Rust source file and extract all Anchor account structs
pub fn parse_anchor_accounts(
    source: &str,
    file_name: &str,
) -> anyhow::Result<Vec<AnchorAccountStruct>> {
    let syntax = syn::parse_file(source)
        .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", file_name, e))?;

    let mut structs = Vec::new();

    for item in &syntax.items {
        if let Item::Struct(item_struct) = item {
            if is_anchor_accounts_struct(item_struct) {
                let parsed = parse_accounts_struct(item_struct, source)?;
                structs.push(parsed);
            }
        }
    }

    Ok(structs)
}

/// Check if a struct derives Accounts (is an Anchor accounts struct)
fn is_anchor_accounts_struct(item: &ItemStruct) -> bool {
    item.attrs.iter().any(|attr| {
        // Check for #[derive(Accounts)]
        if attr.path().is_ident("derive") {
            let tokens = attr.to_token_stream().to_string();
            return tokens.contains("Accounts");
        }
        false
    })
}

/// Parse an Anchor Accounts struct into our model
fn parse_accounts_struct(item: &ItemStruct, source: &str) -> anyhow::Result<AnchorAccountStruct> {
    let struct_name = item.ident.to_string();
    let mut fields = Vec::new();

    if let Fields::Named(named_fields) = &item.fields {
        for field in &named_fields.named {
            let parsed = parse_account_field(field, source)?;
            fields.push(parsed);
        }
    }

    Ok(AnchorAccountStruct {
        name: struct_name,
        fields,
    })
}

/// Parse a single account field
fn parse_account_field(field: &Field, source: &str) -> anyhow::Result<AnchorAccountField> {
    let name = field
        .ident
        .as_ref()
        .map(|i| i.to_string())
        .unwrap_or_default();

    // Get line number - estimate from source by finding field name
    let line_number = source
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains(&name))
        .map(|(idx, _)| idx + 1)
        .unwrap_or(0);

    // Determine the account type
    let account_type = extract_account_type(&field.ty);

    // Parse #[account(...)] constraints
    let constraints = parse_account_constraints(&field.attrs);

    // Check for /// CHECK: comments
    let (has_check_comment, check_reason) = find_check_comment(&field.attrs, source, line_number);

    // Determine security posture
    let security = determine_security(
        &account_type,
        &constraints,
        has_check_comment,
        check_reason.as_deref(),
    );

    Ok(AnchorAccountField {
        name,
        security,
        line_number,
        has_check_comment,
        check_reason,
    })
}

/// Extract the Anchor account type from a field's type annotation
fn extract_account_type(ty: &Type) -> AnchorAccountTypeName {
    let type_str = quote::quote!(#ty).to_string();
    let type_str = type_str.replace(" ", "");

    if type_str.starts_with("Signer") {
        AnchorAccountTypeName::Signer
    } else if type_str.starts_with("Account<") {
        let inner = extract_generic_inner(&type_str, "Account");
        AnchorAccountTypeName::TypedAccount(inner)
    } else if type_str.starts_with("Program<") {
        let inner = extract_generic_inner(&type_str, "Program");
        AnchorAccountTypeName::Program(inner)
    } else if type_str.starts_with("SystemAccount") {
        AnchorAccountTypeName::SystemAccount
    } else if type_str.starts_with("UncheckedAccount") {
        AnchorAccountTypeName::UncheckedAccount
    } else if type_str.starts_with("AccountInfo") {
        AnchorAccountTypeName::AccountInfo
    } else if type_str.starts_with("InterfaceAccount<") {
        let inner = extract_generic_inner(&type_str, "InterfaceAccount");
        AnchorAccountTypeName::InterfaceAccount(inner)
    } else if type_str.starts_with("Interface<") {
        AnchorAccountTypeName::Interface
    } else if type_str.starts_with("Sysvar<") {
        AnchorAccountTypeName::Sysvar
    } else {
        AnchorAccountTypeName::Unknown(type_str)
    }
}

/// Parse the #[account(...)] attribute into constraints
fn parse_account_constraints(attrs: &[Attribute]) -> Vec<AnchorConstraint> {
    let mut constraints = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("account") {
            continue;
        }

        let tokens = attr.to_token_stream().to_string();

        // Remove all whitespace for easier parsing
        let normalized = tokens.replace(" ", "");

        // Parse known constraints from normalized string
        if normalized.contains("seeds") {
            constraints.push(AnchorConstraint::Seeds);
        }
        if normalized.contains("mut") {
            constraints.push(AnchorConstraint::Mutable);
        }
        if normalized.contains("init") {
            constraints.push(AnchorConstraint::Init);
        }
        if normalized.contains("owner=") {
            // Extract the value after owner=
            if let Some(pos) = normalized.find("owner=") {
                let rest = &normalized[pos + 6..];
                let end = rest.find(',').unwrap_or(rest.len());
                let value = rest[..end].to_string();
                constraints.push(AnchorConstraint::Owner(value));
            }
        }
        if normalized.contains("address=") {
            // Extract the value after address=
            if let Some(pos) = normalized.find("address=") {
                let rest = &normalized[pos + 8..];
                let end = rest.find(',').unwrap_or(rest.len());
                let value = rest[..end].to_string();
                constraints.push(AnchorConstraint::Address(value));
            }
        }
        if normalized.contains("has_one=") {
            // Extract the value after has_one=
            if let Some(pos) = normalized.find("has_one=") {
                let rest = &normalized[pos + 8..];
                let end = rest.find(',').unwrap_or(rest.len());
                let value = rest[..end].to_string();
                constraints.push(AnchorConstraint::HasOne(value));
            }
        }
        if normalized.contains("constraint") {
            constraints.push(AnchorConstraint::CustomConstraint(tokens.to_string()));
        }
        if normalized.contains("close=") {
            // Extract the value after close=
            if let Some(pos) = normalized.find("close=") {
                let rest = &normalized[pos + 6..];
                let end = rest.find(',').unwrap_or(rest.len());
                let value = rest[..end].to_string();
                constraints.push(AnchorConstraint::Close(value));
            }
        }
    }

    constraints
}

/// Look for /// CHECK: comment on the lines preceding this field
fn find_check_comment(
    _attrs: &[Attribute],
    source: &str,
    line_number: usize,
) -> (bool, Option<String>) {
    // Look at the 4 lines before this field for a CHECK: comment
    let lines: Vec<&str> = source.lines().collect();
    let start = line_number.saturating_sub(5);
    let end = line_number.min(lines.len());

    if start >= end || end > lines.len() {
        return (false, None);
    }

    for line in &lines[start..end] {
        let trimmed = line.trim();
        if trimmed.starts_with("/// CHECK:") {
            let reason = trimmed
                .strip_prefix("/// CHECK:")
                .unwrap_or("")
                .trim()
                .to_string();
            return (true, Some(reason));
        }
        if trimmed.starts_with("// CHECK:") {
            let reason = trimmed
                .strip_prefix("// CHECK:")
                .unwrap_or("")
                .trim()
                .to_string();
            return (true, Some(reason));
        }
    }

    (false, None)
}

/// Map type + constraints + check comment to a security posture
fn determine_security(
    account_type: &AnchorAccountTypeName,
    constraints: &[AnchorConstraint],
    has_check_comment: bool,
    check_reason: Option<&str>,
) -> AccountSecurity {
    match account_type {
        AnchorAccountTypeName::Signer => AccountSecurity::AnchorSigner,

        AnchorAccountTypeName::TypedAccount(t) => AccountSecurity::AnchorTypedAccount {
            type_name: t.clone(),
        },

        AnchorAccountTypeName::Program(p) => AccountSecurity::AnchorProgram {
            program_name: p.clone(),
        },

        AnchorAccountTypeName::SystemAccount => AccountSecurity::AnchorSystemAccount,

        AnchorAccountTypeName::InterfaceAccount(_)
        | AnchorAccountTypeName::Interface
        | AnchorAccountTypeName::Sysvar => {
            // All are framework-validated types
            AccountSecurity::AnchorTypedAccount {
                type_name: account_type.to_string(),
            }
        }

        // The dangerous types — need to determine if constrained
        AnchorAccountTypeName::AccountInfo | AnchorAccountTypeName::UncheckedAccount => {
            if has_check_comment {
                return AccountSecurity::CheckedByDeveloper {
                    reason: check_reason.unwrap_or("developer verified").to_string(),
                };
            }

            if constraints.is_empty() {
                return AccountSecurity::TrulyUnchecked;
            }

            AccountSecurity::ConstrainedUnchecked {
                constraints: constraints.to_vec(),
            }
        }

        AnchorAccountTypeName::Unknown(_) => {
            // Unknown types — be conservative, treat as unchecked
            if has_check_comment || !constraints.is_empty() {
                AccountSecurity::ConstrainedUnchecked {
                    constraints: constraints.to_vec(),
                }
            } else {
                AccountSecurity::TrulyUnchecked
            }
        }
    }
}

/// Helper types for Anchor account field parsing
#[derive(Debug, Clone)]
pub enum AnchorAccountTypeName {
    /// Signer<'info>
    Signer,
    /// Account<'info, T>
    TypedAccount(String),
    /// Program<'info, T>
    Program(String),
    /// SystemAccount<'info>
    SystemAccount,
    /// AccountInfo<'info>
    AccountInfo,
    /// UncheckedAccount<'info>
    UncheckedAccount,
    /// InterfaceAccount<'info, T>
    InterfaceAccount(String),
    /// Interface
    Interface,
    /// Sysvar<'info, T>
    Sysvar,
    /// Unknown type
    Unknown(String),
}

impl std::fmt::Display for AnchorAccountTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signer => write!(f, "Signer"),
            Self::TypedAccount(t) => write!(f, "Account<{t}>"),
            Self::Program(p) => write!(f, "Program<{p}>"),
            Self::SystemAccount => write!(f, "SystemAccount"),
            Self::AccountInfo => write!(f, "AccountInfo"),
            Self::UncheckedAccount => write!(f, "UncheckedAccount"),
            Self::InterfaceAccount(t) => write!(f, "InterfaceAccount<{t}>"),
            Self::Interface => write!(f, "Interface"),
            Self::Sysvar => write!(f, "Sysvar"),
            Self::Unknown(s) => write!(f, "{s}"),
        }
    }
}

// Helper functions
fn extract_generic_inner(type_str: &str, outer: &str) -> String {
    type_str
        .strip_prefix(&format!("{outer}<"))
        .and_then(|s| s.strip_suffix(">"))
        .unwrap_or("Unknown")
        .to_string()
}

#[allow(dead_code)]
fn extract_constraint_value(tokens: &str, key: &str) -> Option<String> {
    if let Some(pos) = tokens.find(&format!("{key}=")) {
        let rest = &tokens[pos + key.len() + 1..];
        let end = rest.find(',').unwrap_or(rest.len());
        return Some(rest[..end].trim().to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_signer_field() {
        let source = r#"
#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
}
        "#;
        let result = parse_anchor_accounts(source, "test.rs").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Init");
        assert_eq!(result[0].fields.len(), 1);
        assert_eq!(result[0].fields[0].name, "authority");
        assert_eq!(result[0].fields[0].security, AccountSecurity::AnchorSigner);
    }

    #[test]
    fn test_parse_pda_with_seeds() {
        let source = r#"
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,
}
        "#;
        let result = parse_anchor_accounts(source, "test.rs").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fields[0].name, "vault");
        // Should be constrained unchecked with seeds
        match &result[0].fields[0].security {
            AccountSecurity::ConstrainedUnchecked { constraints } => {
                assert!(constraints
                    .iter()
                    .any(|c| matches!(c, AnchorConstraint::Seeds)));
            }
            _ => panic!("Expected ConstrainedUnchecked"),
        }
    }
}
