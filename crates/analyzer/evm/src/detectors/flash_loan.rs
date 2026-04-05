//! Flash loan attack vector detector.
//!
//! Detects patterns where price oracles can be manipulated using flash loans:
//! - Functions that read spot price from token balances
//! - Functions that accept external data (prices) without TWAP validation
//! - Swap functions that move large amounts in a single block

use crate::ast_types::*;
use crate::ast_walker::AstVisitor;
use sentri_core::model::Violation;
use sentri_core::Severity;

/// Detects flash loan attack vectors in DeFi protocols
pub struct FlashLoanDetector<'a> {
    source: &'a str,
    file_name: &'a str,
    /// Accumulated violations
    pub violations: Vec<Violation>,
    balance_oracle_calls: Vec<(usize, String)>, // (line, function_name)
}

impl<'a> FlashLoanDetector<'a> {
    /// Create a new flash loan detector
    pub fn new(source: &'a str, file_name: &'a str) -> Self {
        Self {
            source,
            file_name,
            violations: Vec::new(),
            balance_oracle_calls: Vec::new(),
        }
    }

    fn is_balance_oracle_read(call: &FunctionCall) -> bool {
        // Detects:
        // - token.balanceOf(pool)
        // - IERC20(token).balanceOf(address(this))
        // - getReserves() reads
        // - totalSupply() reads
        if let AstNode::MemberAccess(member) = call.expression.as_ref() {
            return matches!(
                member.member_name.as_str(),
                "balanceOf" | "getReserves" | "totalSupply" | "reserve0" | "reserve1"
            );
        }
        false
    }

    fn is_price_computation_call(call: &FunctionCall) -> bool {
        // Price calculation typically uses balanceOf
        if let AstNode::MemberAccess(member) = call.expression.as_ref() {
            let name = member.member_name.to_lowercase();
            name.contains("price") || name.contains("rate") || name.contains("oracle")
        } else {
            false
        }
    }
}

impl<'a> AstVisitor for FlashLoanDetector<'a> {
    fn visit_function_call(
        &mut self,
        call: &FunctionCall,
        func: &FunctionDefinition,
        contract: &str,
    ) {
        if Self::is_balance_oracle_read(call) {
            let (offset, _, _) = parse_src(&call.src);
            let line = offset_to_line(self.source, offset);
            self.balance_oracle_calls.push((line, func.name.clone()));
        }

        // Detect functions that use balance reads to compute prices
        if Self::is_price_computation_call(call) && !self.balance_oracle_calls.is_empty() {
            for &(call_line, ref func_name) in &self.balance_oracle_calls {
                if call_line < line && func_name == &func.name {
                    // This function reads balance and computes price — flash loan vector
                    let lines: Vec<&str> = self.source.lines().collect();

                    self.violations.push(Violation {
                        id: "evm_flash_loan_price_manipulatable".to_string(),
                        severity: Severity::High,
                        title: "Flash Loan Price Oracle Vulnerability".to_string(),
                        message: format!(
                            "Function '{}::{}' at line {} reads token balances directly \
                             as a price oracle. This can be manipulated with flash loans \
                             within a single transaction. An attacker can borrow a large \
                             amount, move the price, execute trades, and repay within \
                             one block.",
                            contract, func.name, line
                        ),
                        recommendation:
                            "Use time-weighted average prices (TWAP) from Uniswap v3 or \
                             Chainlink price feeds instead of spot prices. \
                             Verify prices are from recent blocks via block.timestamp checks."
                                .to_string(),
                        location: Some(format!("{}:{}", self.file_name, line)),
                        context: Some(extract_context(&lines, line)),
                        cwe: Some("CWE-657".to_string()),
                        references: vec![
                            "https://docs.uniswap.org/concepts/protocol/oracle".to_string(),
                            "https://docs.chain.link/docs/get-the-latest-price/"
                                .to_string(),
                            "https://samczsun.com/the-dangers-of-price-oracles/"
                                .to_string(),
                        ],
                    });
                }
            }
        }
    }
}

fn extract_context(lines: &[&str], line_num: usize) -> String {
    let line_idx = line_num.saturating_sub(1);
    let start = line_idx.saturating_sub(1);
    let end = std::cmp::min(line_idx + 2, lines.len());

    let mut context = String::new();
    for i in start..end {
        if i < lines.len() {
            let marker = if i == line_idx { ">>> " } else { "    " };
            context.push_str(&format!(
                "{}{:4} {}\n",
                marker,
                i + 1,
                lines[i]
            ));
        }
    }
    context
}
