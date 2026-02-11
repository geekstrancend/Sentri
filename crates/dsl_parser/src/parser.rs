//! Parser for invariant DSL expressions.

use crate::grammar::{Grammar, Rule};
use invar_core::model::{BinaryOp, Expression, Invariant};
use invar_core::Result;
use pest::Parser;

/// Parser for invariant DSL.
pub struct InvariantParser;

impl InvariantParser {
    /// Parse a single invariant definition.
    pub fn parse_invariant(input: &str) -> Result<Invariant> {
        let parsed = Grammar::parse(Rule::invariant_def, input)
            .map_err(|e| invar_core::InvarError::ConfigError(e.to_string()))?;

        let invariant_rule = parsed.into_iter().next().ok_or_else(|| {
            invar_core::InvarError::ConfigError("No invariant found".to_string())
        })?;

        let inner = invariant_rule.into_inner();
        let inner_items: Vec<_> = inner.collect();
        
        if inner_items.len() < 2 {
            return Err(invar_core::InvarError::ConfigError(
                "Expected invariant name and expression".to_string(),
            ));
        }

        let name = inner_items[0].as_str().to_string();
        let expression = Self::parse_expr(inner_items[1].clone())?;

        Ok(Invariant {
            name,
            description: None,
            expression,
            severity: "medium".to_string(),
            category: "general".to_string(),
            is_always_true: true,
        })
    }

    fn parse_expr(rule: pest::iterators::Pair<Rule>) -> Result<Expression> {
        use pest::iterators::Pair;

        fn parse_pair(pair: Pair<Rule>) -> Result<Expression> {
            match pair.as_rule() {
                Rule::expr | Rule::logical_or | Rule::logical_and | Rule::comparison | Rule::unary => {
                    let items: Vec<_> = pair.into_inner().collect();
                    if items.is_empty() {
                        return Err(invar_core::InvarError::ConfigError(
                            "Expected expression".to_string(),
                        ));
                    }

                    let mut left = parse_pair(items[0].clone())?;
                    let mut i = 1;

                    while i < items.len() {
                        let operator = &items[i];
                        i += 1;

                        if i >= items.len() {
                            return Err(invar_core::InvarError::ConfigError(
                                "Expected operand after operator".to_string(),
                            ));
                        }

                        let right = parse_pair(items[i].clone())?;
                        i += 1;

                        match operator.as_rule() {
                            Rule::and => {
                                left = Expression::Logical {
                                    left: Box::new(left),
                                    op: invar_core::model::LogicalOp::And,
                                    right: Box::new(right),
                                };
                            }
                            Rule::or => {
                                left = Expression::Logical {
                                    left: Box::new(left),
                                    op: invar_core::model::LogicalOp::Or,
                                    right: Box::new(right),
                                };
                            }
                            Rule::eq => {
                                left = Expression::BinaryOp {
                                    left: Box::new(left),
                                    op: BinaryOp::Eq,
                                    right: Box::new(right),
                                };
                            }
                            Rule::neq => {
                                left = Expression::BinaryOp {
                                    left: Box::new(left),
                                    op: BinaryOp::Neq,
                                    right: Box::new(right),
                                };
                            }
                            Rule::lt => {
                                left = Expression::BinaryOp {
                                    left: Box::new(left),
                                    op: BinaryOp::Lt,
                                    right: Box::new(right),
                                };
                            }
                            Rule::gt => {
                                left = Expression::BinaryOp {
                                    left: Box::new(left),
                                    op: BinaryOp::Gt,
                                    right: Box::new(right),
                                };
                            }
                            Rule::lte => {
                                left = Expression::BinaryOp {
                                    left: Box::new(left),
                                    op: BinaryOp::Lte,
                                    right: Box::new(right),
                                };
                            }
                            Rule::gte => {
                                left = Expression::BinaryOp {
                                    left: Box::new(left),
                                    op: BinaryOp::Gte,
                                    right: Box::new(right),
                                };
                            }
                            _ => {}
                        }
                    }
                    Ok(left)
                }
                Rule::primary => {
                    let mut inner = pair.into_inner();
                    let next = inner.next();
                    if let Some(inner_pair) = next {
                        parse_pair(inner_pair)
                    } else {
                        // Empty primary - should not happen in well-formed grammar
                        Err(invar_core::InvarError::ConfigError(
                            "Unexpected empty primary expression".to_string(),
                        ))
                    }
                }
                Rule::function_call => {
                    let items: Vec<_> = pair.into_inner().collect();
                    if items.is_empty() {
                        return Err(invar_core::InvarError::ConfigError(
                            "Expected function name".to_string(),
                        ));
                    }
                    let name = items[0].as_str().to_string();
                    let args: Result<Vec<_>> = items[1..].iter().map(|arg| parse_pair(arg.clone())).collect();
                    Ok(Expression::FunctionCall {
                        name,
                        args: args?,
                    })
                }
                Rule::boolean => {
                    let val = pair.as_str() == "true";
                    Ok(Expression::Boolean(val))
                }
                Rule::integer => {
                    let val = pair
                        .as_str()
                        .parse::<i128>()
                        .map_err(|_| invar_core::InvarError::ConfigError("Invalid integer".to_string()))?;
                    Ok(Expression::Int(val))
                }
                Rule::identifier => Ok(Expression::Var(pair.as_str().to_string())),
                _ => Err(invar_core::InvarError::ConfigError(format!(
                    "Unexpected rule: {:?}",
                    pair.as_rule()
                ))),
            }
        }

        parse_pair(rule)
    }
}

/// Parse a complete invariant definition string.
pub fn parse_invariant(input: &str) -> Result<Invariant> {
    InvariantParser::parse_invariant(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_invariant() {
        let input = r#"invariant BalancePositive { balance >= 0 }"#;
        let result = parse_invariant(input);
        if let Err(ref e) = result {
            eprintln!("Parse error: {}", e);
        }
        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.name, "BalancePositive");
    }

    #[test]
    fn test_parse_invariant_with_and() {
        let input = r#"invariant MultiCondition { balance >= 0 && total_supply > 0 }"#;
        let result = parse_invariant(input);
        if let Err(ref e) = result {
            eprintln!("Parse error: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_invariant_no_expression() {
        let input = r#"invariant Empty { }"#;
        let result = parse_invariant(input);
        assert!(result.is_err());
    }
}
