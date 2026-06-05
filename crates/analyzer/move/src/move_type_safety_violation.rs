/// Move Type Safety Violation Detector
///
/// Detects H52 vulnerability: Type safety violations in Move
///
/// The vulnerability occurs when:
/// 1. Generic types misused with incompatible assets
/// 2. Resource capabilities mixed incorrectly
/// 3. Unsafe casting or type confusion
/// 4. Can lead to fund loss or privilege escalation
///

use lazy_static::lazy_static;
use regex::Regex;
use sentri_core::Finding;

lazy_static! {
    static ref GENERIC_TYPE: Regex = 
        Regex::new(r"(?i)<T\s*:|<Token\s*:|generic\s*type").unwrap();
    static ref UNSAFE_AS: Regex = 
        Regex::new(r"(?i)as\s*(&?[A-Z]\w*<|T|Coin)").unwrap();
    static ref RESOURCE_CAST: Regex = 
        Regex::new(r"(?i)(Coin|Asset|Token)<.*>\s*as\s*").unwrap();
    static ref TYPE_ANNOTATION: Regex = 
        Regex::new(r"(?i)let.*:\s*&?\w+<.*>").unwrap();
}

pub fn detect_move_type_safety_violation(source: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        if line.trim().starts_with("//") || !UNSAFE_AS.is_match(line) {
            continue;
        }

        let context_end = std::cmp::min(line_num + 50, source.lines().count());
        let function_body = source
            .lines()
            .skip(line_num)
            .take(context_end - line_num)
            .collect::<Vec<_>>()
            .join("\n");

        let has_generic = GENERIC_TYPE.is_match(&function_body);
        let has_type_annotation = TYPE_ANNOTATION.is_match(&function_body);

        if has_generic && !has_type_annotation {
            findings.push(
                Finding::new(
                    "move_type_safety_violation".to_string(),
                    sentri_core::Severity::Medium,
                    file_path.to_string(),
                    line_num + 1,
                    0,
                    "Unsafe type casting in Move. Verify type safety with explicit annotations.".to_string(),
                    line.trim().to_string(),
                )
                .with_metadata("exploit_id", "H52")
                .with_metadata("exploit_name", "Move Type Safety")
                .with_metadata("loss", "$2.1M")
                .with_metadata("year", "2023")
                .with_metadata("vulnerability_type", "type_safety")
                .with_metadata("detector", "pattern_analysis")
                .with_metadata("remediation", "Use explicit type annotations and avoid unsafe casts"),
            );
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_type_cast() {
        let vulnerable = r#"
        fun swap<T>(asset: T) {
            let coin = asset as Coin<USDC>;  // Unsafe cast!
            transfer(coin);
        }
        "#;
        let findings = detect_move_type_safety_violation(vulnerable, "test.move");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_safe_generic_annotation() {
        let safe = r#"
        fun swap<T: Coin>(asset: T) {
            let amount: u64 = get_balance<T>(&asset);
            deposit<T>(asset, amount);
        }
        "#;
        let findings = detect_move_type_safety_violation(safe, "test.move");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_explicit_type_annotation() {
        let safe = r#"
        fun process<T>(value: T) {
            let x: CoinValue<T> = read_value(&value);
            use_value(&x);
        }
        "#;
        let findings = detect_move_type_safety_violation(safe, "test.move");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_resource_constraint() {
        let safe = r#"
        fun transfer<T: drop>(coin: T) {
            let value: Coin<T> = coin;
            send_coin(value);
        }
        "#;
        let findings = detect_move_type_safety_violation(safe, "test.move");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_generic_function_call() {
        let safe = r#"
        fun deposit_token<CoinType>(
            user_account: &signer,
            amount: u64,
        ) {
            let coin = coin::withdraw<CoinType>(user_account, amount);
            register_coin<CoinType>();
        }
        "#;
        let findings = detect_move_type_safety_violation(safe, "test.move");
        assert!(findings.is_empty());
    }
}
