/// Represents the security posture of an account field
/// in an Anchor Accounts struct.
#[derive(Debug, Clone, PartialEq)]
pub enum AccountSecurity {
    /// Signer<'info> — Anchor enforces signature automatically
    AnchorSigner,
    /// Account<'info, T> — Anchor enforces ownership + deserialization
    AnchorTypedAccount {
        /// The account type name
        type_name: String,
    },
    /// AccountInfo<'info> or UncheckedAccount<'info> with constraints
    ConstrainedUnchecked {
        /// Security constraints applied
        constraints: Vec<AnchorConstraint>,
    },
    /// AccountInfo<'info> or UncheckedAccount<'info> with CHECK: comment
    CheckedByDeveloper {
        /// Reason from CHECK: comment
        reason: String,
    },
    /// AccountInfo<'info> or UncheckedAccount<'info> with no validation
    TrulyUnchecked,
    /// Program<'info, T> — validated by Anchor
    AnchorProgram {
        /// The program type name
        program_name: String,
    },
    /// SystemAccount<'info> — validated by Anchor
    AnchorSystemAccount,
}

/// Constraints extracted from #[account(...)] attributes
#[derive(Debug, Clone, PartialEq)]
pub enum AnchorConstraint {
    /// seeds = [...] — PDA derivation validated
    Seeds,
    /// owner = ... — explicit owner check
    Owner(String),
    /// address = ... — exact address check  
    Address(String),
    /// constraint = ... — custom constraint expression
    CustomConstraint(String),
    /// mut — mutable (not a security constraint)
    Mutable,
    /// init — initialization (security relevant)
    Init,
    /// close = ... — closing account
    Close(String),
    /// has_one = ... — field equality check
    HasOne(String),
}

/// A parsed account field from an Anchor Accounts struct
#[derive(Debug, Clone)]
pub struct AnchorAccountField {
    /// Field name
    pub name: String,
    /// Security classification of this account
    pub security: AccountSecurity,
    /// Line number in source code
    pub line_number: usize,
    /// Whether a CHECK: comment was present
    pub has_check_comment: bool,
    /// Reason from CHECK: comment if present
    pub check_reason: Option<String>,
}

impl AccountSecurity {
    /// Returns true if Anchor handles validation automatically
    pub fn is_framework_validated(&self) -> bool {
        matches!(
            self,
            AccountSecurity::AnchorSigner
                | AccountSecurity::AnchorTypedAccount { .. }
                | AccountSecurity::AnchorProgram { .. }
                | AccountSecurity::AnchorSystemAccount
        )
    }

    /// Returns true if there are sufficient manual constraints
    pub fn is_adequately_constrained(&self) -> bool {
        match self {
            AccountSecurity::ConstrainedUnchecked { constraints } => {
                // Seeds alone is sufficient for PDAs
                // Owner check alone is sufficient
                // Address check alone is sufficient
                constraints.iter().any(|c| {
                    matches!(
                        c,
                        AnchorConstraint::Seeds
                            | AnchorConstraint::Owner(_)
                            | AnchorConstraint::Address(_)
                    )
                })
            }
            AccountSecurity::CheckedByDeveloper { .. } => true,
            _ => false,
        }
    }

    /// Returns the violation severity for this account
    pub fn violation_severity(&self) -> Option<&'static str> {
        match self {
            // Framework handles it — no violation
            s if s.is_framework_validated() => None,
            // Developer has constrained it adequately — no violation
            s if s.is_adequately_constrained() => None,
            // Unchecked with no constraints — critical
            AccountSecurity::TrulyUnchecked => Some("critical"),
            // Everything else — low (belt and suspenders suggestion)
            _ => Some("low"),
        }
    }
}

/// A parsed Anchor Accounts struct
#[derive(Debug, Clone)]
pub struct AnchorAccountStruct {
    /// Struct name
    pub name: String,
    /// Account fields in this struct
    pub fields: Vec<AnchorAccountField>,
}
