//! Chain-agnostic semantic-model extraction for Move modules.
//!
//! Move has no `syn`-equivalent structural parser wired in yet (bringing one
//! online is follow-on work under Epic 6.1/6.2's pattern, tracked
//! separately), so this extractor works over signature-line text like the
//! rest of the Move analyzer does today. The point of this module isn't
//! parser quality — it's proving that the same chain-agnostic rule in
//! [`sentri_ir::rules`] fires correctly once Move populates the shared
//! [`SemanticModel`], with zero rule-side code changed to support it.

use regex::Regex;
use sentri_ir::{
    AuthCheckKind, AuthorizationCheck, MutationKind, PrivilegedMutation, SemanticModel,
};

/// Function name substrings that indicate a privileged mutation, paired
/// with the mutation category they represent.
const SENSITIVE_FUNCTIONS: &[(&str, MutationKind)] = &[
    ("withdraw", MutationKind::FundTransfer),
    ("transfer", MutationKind::FundTransfer),
    ("mint", MutationKind::FundTransfer),
    ("burn", MutationKind::FundTransfer),
    ("upgrade", MutationKind::Upgrade),
];

/// Build a chain-agnostic semantic model from Move module source.
///
/// A capability-typed parameter (`&AdminCap`, `&Capability<...>`, ...) is
/// treated as the guard: Move's capability pattern is its equivalent of an
/// authorization check — possession of the typed value at the call site
/// stands in for a signer/role check.
pub fn build_semantic_model(source: &str, file_path: &str) -> SemanticModel {
    let mut model = SemanticModel::new("move", file_path);

    // Matched against the whole source (not line-by-line): a real-world Move
    // function signature commonly wraps its parameter list across multiple
    // lines, which a per-line regex would silently never match at all.
    // `[\s\S]*?` (rather than `[^)]*`) lets the parameter capture span
    // newlines; it's non-greedy so it still stops at the first `)`.
    let sig_re = Regex::new(r"public\s+(?:entry\s+)?fun\s+(\w+)\s*(?:<[^>]*>)?\s*\(([\s\S]*?)\)")
        .expect("valid regex");
    let cap_re = Regex::new(r"&(?:mut\s+)?(\w*Cap\w*)").expect("valid regex");

    for caps in sig_re.captures_iter(source) {
        let fn_name = &caps[1];
        let params = &caps[2];

        let lower = fn_name.to_lowercase();
        let Some((_, kind)) = SENSITIVE_FUNCTIONS
            .iter()
            .find(|(needle, _)| lower.contains(needle))
        else {
            continue;
        };

        let guards: Vec<AuthorizationCheck> = cap_re
            .captures_iter(params)
            .map(|c| AuthorizationCheck {
                kind: AuthCheckKind::RoleOrCapability,
                source: c[1].to_string(),
            })
            .collect();

        let name_match = caps
            .get(1)
            .expect("group 1 always matches with the outer regex");
        model.mutations.push(PrivilegedMutation {
            entry_point: fn_name.to_string(),
            kind: kind.clone(),
            line: offset_to_line(source, name_match.start()),
            guards,
        });
    }

    model
}

/// 1-indexed line number containing the given byte offset.
fn offset_to_line(source: &str, byte_offset: usize) -> usize {
    source
        .get(..byte_offset.min(source.len()))
        .unwrap_or("")
        .matches('\n')
        .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentri_ir::rules::find_unauthorized_privileged_mutations;

    const FIXTURE: &str = r#"
module vault::vault {
    public entry fun withdraw<T>(vault: &mut Vault<T>, amount: u64, ctx: &mut TxContext) {
        // no capability check
    }

    public entry fun admin_withdraw<T>(admin: &AdminCap, vault: &mut Vault<T>, amount: u64) {
        // guarded by capability
    }
}
"#;

    #[test]
    fn flags_withdraw_with_no_guard_but_not_admin_withdraw() {
        let model = build_semantic_model(FIXTURE, "vault.move");
        assert_eq!(model.chain, "move");
        assert_eq!(model.mutations.len(), 2);

        let findings = find_unauthorized_privileged_mutations(&model);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("withdraw"));
        assert!(!findings[0].message.contains("admin_withdraw"));
    }

    /// A per-line regex would never match this at all: real-world Move
    /// signatures commonly wrap a long parameter list across several lines.
    const MULTILINE_FIXTURE: &str = r#"
module vault::vault {
    public entry fun withdraw<T>(
        vault: &mut Vault<T>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // no capability check
    }

    public entry fun admin_withdraw<T>(
        admin: &AdminCap,
        vault: &mut Vault<T>,
        amount: u64
    ) {
        // guarded by capability
    }
}
"#;

    #[test]
    fn flags_multiline_signature_with_no_guard() {
        let model = build_semantic_model(MULTILINE_FIXTURE, "vault.move");
        assert_eq!(model.mutations.len(), 2);

        let findings = find_unauthorized_privileged_mutations(&model);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("withdraw"));
        assert!(!findings[0].message.contains("admin_withdraw"));

        // The reported line should point at `withdraw`'s own signature line,
        // not get thrown off by the multi-line parameter list.
        let withdraw_mutation = model
            .mutations
            .iter()
            .find(|m| m.entry_point == "withdraw")
            .unwrap();
        let expected_line = MULTILINE_FIXTURE
            .lines()
            .position(|l| l.contains("fun withdraw"))
            .unwrap()
            + 1;
        assert_eq!(withdraw_mutation.line, expected_line);
    }
}
