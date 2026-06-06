//! Detector for LST (Liquid Staking Token) depeg collateral risk vulnerabilities.
//!
//! This detector identifies when protocols accept Liquid Staking Tokens (stETH, rsETH, etc.)
//! as collateral without depeg protection. This pattern was exploited in H47 KelpDAO/rsETH
//! ($292M, 2026) when rsETH depegged from ETH.

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    /// List of known liquid staking tokens
    static ref KNOWN_LSTS: Vec<&'static str> = vec![
        "stETH", "rETH", "eETH", "rsETH", "mETH", "osETH",
        "sfrxETH", "swETH", "wstETH", "ankrETH", "cbETH", "frxETH",
        "lsETH", "SETH", "aETH",
    ];

    /// Regex to match borrow/deposit functions
    static ref BORROW_DEPOSIT_REGEX: Regex =
        Regex::new(r"(?i)function\s+(borrow|deposit|supply|collateralize)\s*\(").unwrap();

    /// Regex to match oracle price functions
    static ref PRICE_REGEX: Regex =
        Regex::new(r"(?i)(price|rate|exchange|value)").unwrap();
}

/// Detects LST depeg collateral risk in lending protocols.
pub fn detect_lst_depeg_collateral_risk(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Pattern 1: Find borrow/deposit functions that accept collateral
    for (func_line_num, func_line) in source.lines().enumerate() {
        if !BORROW_DEPOSIT_REGEX.is_match(func_line) {
            continue;
        }

        let func_name = extract_function_name(func_line);

        // Extract function body (~50 lines)
        let func_start = func_line_num;
        let func_end = (func_line_num + 50).min(source.lines().count());
        let func_body = source
            .lines()
            .skip(func_start)
            .take(func_end - func_start)
            .collect::<Vec<&str>>()
            .join("\n");

        // Pattern 2: Check if function accepts LST and lacks depeg protection
        for lst in KNOWN_LSTS.iter() {
            if func_body.to_lowercase().contains(&lst.to_lowercase())
                && !has_depeg_protection(&func_body)
            {
                let message = format!(
                        "Function '{}' accepts {} as collateral without depeg protection. \
                         Liquid staking tokens can depeg from their underlying asset. \
                         H47 KelpDAO ($292M) was exploited when rsETH depegged 10%+ from ETH, \
                         causing cascade liquidations of all positions using rsETH as collateral. \
                         \
                         Attack flow: \
                         1. Large LST depeg event occurs (e.g., staking provider issues) \
                         2. LST price drops 5-15% relative to ETH \
                         3. Protocol liquidates all positions using LST, realizing large losses \
                         4. Market panic cascades into full insolvency \
                         \
                         Required fix: Add depeg band validation: \
                         require(lstPrice >= underlyingPrice * 0.95, \"LST depegged beyond threshold\"); \
                         Recommended: Add dynamic risk multiplier to collateral value based on depeg risk.",
                        func_name, lst
                    );

                findings.push(
                    Finding::new(
                        "evm_lst_depeg_collateral_risk".to_string(),
                        sentri_core::Severity::Critical,
                        file_path.to_string(),
                        func_line_num + 1,
                        0,
                        message,
                        func_line.trim().to_string(),
                    )
                    .with_metadata("exploit_id".to_string(), "H47".to_string())
                    .with_metadata(
                        "exploit_name".to_string(),
                        "KelpDAO/rsETH Depeg".to_string(),
                    )
                    .with_metadata("loss".to_string(), "$292M".to_string())
                    .with_metadata("year".to_string(), "2026".to_string())
                    .with_metadata("token".to_string(), lst.to_string())
                    .with_metadata(
                        "vulnerability_type".to_string(),
                        "depeg_cascade".to_string(),
                    )
                    .with_metadata("detector".to_string(), "oracle_risk_analysis".to_string())
                    .with_source_fragment(func_body.clone()),
                );
            }
        }
    }

    findings
}

/// Extract function name from function declaration
fn extract_function_name(line: &str) -> String {
    if let Some(start) = line.find("function ") {
        let after_function = &line[start + 9..];
        if let Some(end) = after_function.find('(') {
            return after_function[..end].trim().to_string();
        }
    }
    "collateralize".to_string()
}

/// Check if function has depeg protection
fn has_depeg_protection(func_body: &str) -> bool {
    let func_lower = func_body.to_lowercase();

    // Patterns indicating depeg checks
    let depeg_patterns = vec![
        "depeg",
        "peg_band",
        "max_deviation",
        "deviation_tolerance",
        "peg_threshold",
        "price_band",
        "ratio_band",
        "0.95", // 95% peg band
        "0.9",  // 90% peg band
        "require(.*price",
        "max_collateral_ratio",
    ];

    depeg_patterns
        .iter()
        .any(|pattern| func_lower.contains(&pattern.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulnerable_no_depeg_protection_kelp_dao() {
        let code = r#"
            contract KelpDAO {
                function deposit(uint256 amount, address token) external {
                    require(supportedTokens[token], "Unsupported token");
                    // Accepts rsETH without any depeg check!
                    if (token == rsETH) {
                        userCollateral[msg.sender][token] += amount;
                    }
                }
                
                function borrow(uint amount) external {
                    // Can borrow against rsETH that later depegs
                    require(getHealthFactor(msg.sender) >= 1.5);
                    _borrow(amount);
                }
            }
        "#;

        let findings = detect_lst_depeg_collateral_risk(code, "kelpDAO.sol");
        assert!(!findings.is_empty(), "Should detect LST depeg risk");
        assert!(findings[0].invariant_id.contains("lst_depeg"));
    }

    #[test]
    fn test_vulnerable_steth_no_band() {
        let code = r#"
            contract Lender {
                function deposit(uint amount, address token) external {
                    if (token == stETH) {
                        collateral[msg.sender] += amount;  // No depeg check!
                    }
                }
            }
        "#;

        let findings = detect_lst_depeg_collateral_risk(code, "lender.sol");
        assert!(
            !findings.is_empty(),
            "Should detect stETH without depeg protection"
        );
    }

    #[test]
    fn test_safe_with_depeg_band() {
        let code = r#"
            contract SafeLender {
                function deposit(uint amount, address token) external {
                    if (token == rsETH) {
                        uint ethPrice = getPrice(ETH);
                        uint lstPrice = getPrice(rsETH);
                        
                        // Check depeg band - allow max 5% deviation
                        require(lstPrice >= (ethPrice * 95) / 100, "LST depegged");
                        
                        collateral[msg.sender][token] += amount;
                    }
                }
            }
        "#;

        let findings = detect_lst_depeg_collateral_risk(code, "safe.sol");
        assert!(findings.is_empty(), "Should not flag with depeg band");
    }

    #[test]
    fn test_safe_with_peg_threshold() {
        let code = r#"
            contract SafeProtocol {
                uint constant PEG_BAND = 0.9;  // Allow 10% deviation
                
                function deposit(uint amount, address token) external {
                    require(supportedLSTs[token], "Not LST");
                    
                    uint deviation = calculateDeviation(token);
                    require(deviation < (1 - PEG_BAND), "Too much depeg");
                    
                    userCollateral[msg.sender][token] += amount;
                }
            }
        "#;

        let findings = detect_lst_depeg_collateral_risk(code, "safe.sol");
        assert!(findings.is_empty(), "Should detect peg_band check");
    }

    #[test]
    fn test_vulnerable_multiple_lsts() {
        let code = r#"
            contract MultiLST {
                function deposit(address[] calldata tokens, uint[] calldata amounts) external {
                    for (uint i = 0; i < tokens.length; i++) {
                        // Accept stETH, rETH, rsETH without any depeg checks
                        collateral[msg.sender] += amounts[i];
                    }
                }
            }
        "#;

        let findings = detect_lst_depeg_collateral_risk(code, "multilst.sol");
        assert!(
            !findings.is_empty(),
            "Should detect LST depeg risk for multiple tokens"
        );
    }

    #[test]
    fn test_non_lst_token() {
        let code = r#"
            contract TokenProtocol {
                function deposit(uint amount, address token) external {
                    if (token == USDC) {
                        collateral[msg.sender] += amount;  // USDC doesn't depeg like LSTs
                    }
                }
            }
        "#;

        let findings = detect_lst_depeg_collateral_risk(code, "token.sol");
        assert!(findings.is_empty(), "Should not flag non-LST tokens");
    }
}
