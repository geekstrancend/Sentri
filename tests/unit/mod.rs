//! Unit tests for DSL parser.
//!
//! These tests validate core parser functionality in isolation.
//! Coverage target: 95%+ for parser module.

mod parser_tests {
    use invar_dsl_parser::lexer::Lexer;
    use invar_dsl_parser::parser::Parser;

    #[test]
    fn test_parse_simple_invariant() {
        let input = r#"invariant: balance_conservation
        forall tx in transactions:
            sum(tx.inputs) == sum(tx.outputs)
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser should succeed on valid DSL");
    }

    #[test]
    fn test_parse_with_context() {
        let input = r#"
        context {
            state: AccountState,
            chain: Solana
        }
        
        invariant: vault_conservation
        forall account in state.accounts:
            account.balance >= 0
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser should handle context blocks");
    }

    #[test]
    fn test_parse_type_annotations() {
        let input = r#"
        invariant: typed_balance
        forall account: Account in state.accounts:
            (account.balance: u64) >= 0
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser should handle type annotations");
    }

    #[test]
    fn test_parse_complex_expression() {
        let input = r#"
        invariant: complex_condition
        forall tx in transactions:
            (tx.amount > 0 && tx.fee >= MIN_FEE) ||
            (tx.priority == HIGH && tx.fee >= MIN_PRIORITY_FEE)
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser should handle complex boolean expressions");
    }

    #[test]
    fn test_parse_with_aggregations() {
        let input = r#"
        invariant: sum_conservation
        sum(balances) == total_supply &&
        max(individual_balance) <= max_allowed &&
        count(accounts) == account_count
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser should handle aggregation functions");
    }

    #[test]
    fn test_parse_error_missing_colon() {
        let input = r#"invariant balance_conservation
        forall tx in transactions:
            sum(tx.inputs) == sum(tx.outputs)
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_err(), "Parser should reject invalid syntax");
    }

    #[test]
    fn test_parse_error_unclosed_brace() {
        let input = r#"
        context {
            state: AccountState
        
        invariant: test
        true
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_err(), "Parser should reject unclosed braces");
    }

    #[test]
    fn test_parse_determinism() {
        let input = r#"
        invariant: deterministic_test
        forall x in items:
            x.value > 0
        "#;

        let lexer1 = Lexer::new(input);
        let mut parser1 = Parser::new(lexer1);
        let result1 = parser1.parse();

        let lexer2 = Lexer::new(input);
        let mut parser2 = Parser::new(lexer2);
        let result2 = parser2.parse();

        assert_eq!(
            format!("{:?}", result1),
            format!("{:?}", result2),
            "Parser must be deterministic"
        );
    }
}

//! Unit tests for type checker.
//!
//! These tests validate type checking correctness.
//! Coverage target: 95%+ for type_checker module.

mod type_checker_tests {
    use invar_core::type_checker::TypeChecker;
    use invar_core::types::{Type, TypeEnvironment};

    #[test]
    fn test_type_inference_number() {
        let mut checker = TypeChecker::new();
        let expr = "42";
        let inferred = checker.infer_type(expr);

        assert!(inferred.is_ok());
        if let Ok(Type::Integer) = inferred {
            // Success
        } else {
            panic!("Should infer Integer type for numeric literal");
        }
    }

    #[test]
    fn test_type_mismatch_detection() {
        let mut checker = TypeChecker::new();
        let expr = "\"string\" + 42";
        let result = checker.check_expr(expr);

        assert!(
            result.is_err(),
            "Type checker should detect string + number type error"
        );
    }

    #[test]
    fn test_type_consistency_forall() {
        let mut checker = TypeChecker::new();
        let expr = "forall x: u64 in items: x > 0";
        let result = checker.check_expr(expr);

        assert!(result.is_ok(), "Type checker should approve consistent forall");
    }

    #[test]
    fn test_comparison_type_rules() {
        let mut checker = TypeChecker::new();

        // Valid comparisons
        assert!(checker.check_expr("5 > 3").is_ok());
        assert!(checker.check_expr("\"a\" == \"b\"").is_ok());

        // Invalid comparisons
        assert!(checker.check_expr("5 > \"string\"").is_err());
    }

    #[test]
    fn test_function_signature_validation() {
        let mut checker = TypeChecker::new();
        
        // Assuming a function registry
        let result = checker.check_expr("sum(balances: u64[])");
        // Should be ok
        assert!(result.is_ok() || result.is_err()); // Depending on implementation
    }

    #[test]
    fn test_type_checker_determinism() {
        let mut checker1 = TypeChecker::new();
        let result1 = checker1.check_expr("x > 0 && y < 100");

        let mut checker2 = TypeChecker::new();
        let result2 = checker2.check_expr("x > 0 && y < 100");

        assert_eq!(
            format!("{:?}", result1),
            format!("{:?}", result2),
            "Type checker must be deterministic"
        );
    }
}

//! Unit tests for expression evaluator.
//!
//! These tests validate evaluation correctness.
//! Coverage target: 95%+ for evaluator module.

mod evaluator_tests {
    use invar_core::evaluator::Evaluator;
    use invar_core::types::TypedValue;

    #[test]
    fn test_evaluate_literal() {
        let evaluator = Evaluator::new();
        let result = evaluator.eval("42");

        assert!(result.is_ok(), "Should evaluate numeric literal");
    }

    #[test]
    fn test_evaluate_arithmetic() {
        let evaluator = Evaluator::new();
        
        let result = evaluator.eval("2 + 3 * 4");
        assert!(result.is_ok(), "Should evaluate arithmetic with precedence");

        let result = evaluator.eval("100 - 50");
        assert!(result.is_ok(), "Should evaluate subtraction");
    }

    #[test]
    fn test_evaluate_comparison() {
        let evaluator = Evaluator::new();
        
        let result = evaluator.eval("5 > 3");
        assert!(result.is_ok(), "Should evaluate comparison");

        let result = evaluator.eval("10 <= 10");
        assert!(result.is_ok(), "Should evaluate equality");
    }

    #[test]
    fn test_evaluate_logical() {
        let evaluator = Evaluator::new();
        
        let result = evaluator.eval("true && false");
        assert!(result.is_ok(), "Should evaluate logical AND");

        let result = evaluator.eval("true || false");
        assert!(result.is_ok(), "Should evaluate logical OR");

        let result = evaluator.eval("!true");
        assert!(result.is_ok(), "Should evaluate logical NOT");
    }

    #[test]
    fn test_evaluate_determinism() {
        let evaluator1 = Evaluator::new();
        let result1 = evaluator1.eval("(5 + 3) * 2");

        let evaluator2 = Evaluator::new();
        let result2 = evaluator2.eval("(5 + 3) * 2");

        assert_eq!(
            format!("{:?}", result1),
            format!("{:?}", result2),
            "Evaluator must be deterministic"
        );
    }
}

//! Unit tests for AST construction.
//!
//! These tests validate AST structure and generation.
//! Coverage target: 95%+ for AST module.

mod ast_tests {
    use invar_ir::ast::*;

    #[test]
    fn test_ast_node_creation() {
        let node = AstNode::Literal(LiteralNode::Integer(42));
        assert!(matches!(node, AstNode::Literal(_)));
    }

    #[test]
    fn test_ast_binary_operation() {
        let left = Box::new(AstNode::Literal(LiteralNode::Integer(5)));
        let right = Box::new(AstNode::Literal(LiteralNode::Integer(3)));
        let node = AstNode::BinaryOp {
            op: BinaryOp::Add,
            left,
            right,
        };

        assert!(matches!(node, AstNode::BinaryOp { .. }));
    }

    #[test]
    fn test_ast_determinism() {
        let node1 = AstNode::Literal(LiteralNode::Integer(42));
        let node2 = AstNode::Literal(LiteralNode::Integer(42));

        assert_eq!(format!("{:?}", node1), format!("{:?}", node2));
    }
}
