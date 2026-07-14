//! AST visitor pattern for walking and analyzing Solidity AST.
//!
//! The walker uses the visitor pattern to enable custom analyses on the AST.
//! Implementations can override specific visit/leave methods for the node types
//! they care about.

use crate::ast_types::*;

/// Maximum expression nesting depth the walker will descend into. Guards
/// against stack overflow on pathologically nested expressions (e.g. a
/// contract with thousands of parenthesized/binary-op levels); AST nodes
/// beyond this depth are simply not visited rather than causing a crash.
const MAX_WALK_DEPTH: usize = 256;

/// Trait for visiting AST nodes
///
/// Implement this trait to perform analysis on different node types.
/// The walker will call appropriate methods as it traverses the AST.
pub trait AstVisitor {
    /// Called when entering a function definition
    fn visit_function(&mut self, _func: &FunctionDefinition, _contract: &str) {}

    /// Called when a function call is encountered
    fn visit_function_call(
        &mut self,
        _call: &FunctionCall,
        _containing_function: &FunctionDefinition,
        _contract: &str,
    ) {
    }

    /// Called when an assignment is encountered
    fn visit_assignment(
        &mut self,
        _assign: &Assignment,
        _containing_function: &FunctionDefinition,
        _contract: &str,
    ) {
    }

    /// Called for each binary operation
    fn visit_binary_op(
        &mut self,
        _op: &BinaryOperation,
        _containing_function: &FunctionDefinition,
        _contract: &str,
    ) {
    }

    /// Called when visiting a state variable declaration
    fn visit_state_variable(&mut self, _var: &VariableDeclaration, _contract: &str) {}

    /// Called after all nodes in a function are visited
    fn leave_function(
        &mut self,
        _func: &FunctionDefinition,
        _statements: &[AstNode],
        _contract: &str,
    ) {
    }

    /// Called when visiting a member access (e.g., obj.member)
    fn visit_member_access(&mut self, _access: &MemberAccess, _contract: &str) {}

    /// Called when visiting an identifier
    fn visit_identifier(&mut self, _id: &Identifier, _contract: &str) {}
}

/// Walks the AST and calls visitor methods
pub struct AstWalker<'a> {
    visitor: &'a mut dyn AstVisitor,
}

impl<'a> AstWalker<'a> {
    /// Create a new AST walker with the given visitor
    pub fn new(visitor: &'a mut dyn AstVisitor) -> Self {
        Self { visitor }
    }

    /// Start walking a source unit
    pub fn walk_source_unit(&mut self, unit: &SourceUnit) {
        for node in &unit.nodes {
            self.walk_node(node, None, "", 0);
        }
    }

    fn walk_node(
        &mut self,
        node: &AstNode,
        current_func: Option<&FunctionDefinition>,
        current_contract: &str,
        depth: usize,
    ) {
        if depth >= MAX_WALK_DEPTH {
            return;
        }
        let depth = depth + 1;

        match node {
            AstNode::ContractDefinition(contract) => {
                self.walk_contract(contract);
            }
            AstNode::FunctionDefinition(func) => {
                self.visitor.visit_function(func, current_contract);
                if let Some(body) = &func.body {
                    for stmt in &body.statements {
                        self.walk_node(stmt, Some(func), current_contract, depth);
                    }
                    self.visitor
                        .leave_function(func, &body.statements, current_contract);
                }
            }
            AstNode::FunctionCall(call) => {
                if let Some(func) = current_func {
                    self.visitor
                        .visit_function_call(call, func, current_contract);
                }
                // Walk into call arguments
                for arg in &call.arguments {
                    self.walk_node(arg, current_func, current_contract, depth);
                }
                self.walk_node(&call.expression, current_func, current_contract, depth);
            }
            AstNode::Assignment(assign) => {
                if let Some(func) = current_func {
                    self.visitor
                        .visit_assignment(assign, func, current_contract);
                }
                self.walk_node(
                    &assign.left_hand_side,
                    current_func,
                    current_contract,
                    depth,
                );
                self.walk_node(
                    &assign.right_hand_side,
                    current_func,
                    current_contract,
                    depth,
                );
            }
            AstNode::BinaryOperation(op) => {
                if let Some(func) = current_func {
                    self.visitor.visit_binary_op(op, func, current_contract);
                }
                self.walk_node(&op.left_expression, current_func, current_contract, depth);
                self.walk_node(&op.right_expression, current_func, current_contract, depth);
            }
            AstNode::Block(block) => {
                for stmt in &block.statements {
                    self.walk_node(stmt, current_func, current_contract, depth);
                }
            }
            AstNode::ExpressionStatement(expr) => {
                if let Some(expression) = &expr.expression {
                    self.walk_node(expression, current_func, current_contract, depth);
                }
            }
            AstNode::IfStatement(if_stmt) => {
                self.walk_node(&if_stmt.condition, current_func, current_contract, depth);
                self.walk_node(&if_stmt.true_body, current_func, current_contract, depth);
                if let Some(false_body) = &if_stmt.false_body {
                    self.walk_node(false_body, current_func, current_contract, depth);
                }
            }
            AstNode::StateVariableDeclaration(decl) => {
                for var in &decl.variables {
                    self.visitor.visit_state_variable(var, current_contract);
                }
            }
            AstNode::MemberAccess(access) => {
                self.visitor.visit_member_access(access, current_contract);
                self.walk_node(&access.expression, current_func, current_contract, depth);
            }
            AstNode::Identifier(id) => {
                self.visitor.visit_identifier(id, current_contract);
            }
            AstNode::IndexAccess(idx) => {
                self.walk_node(&idx.base_expression, current_func, current_contract, depth);
                if let Some(index) = &idx.index_expression {
                    self.walk_node(index, current_func, current_contract, depth);
                }
            }
            AstNode::ForStatement(_) | AstNode::WhileStatement(_) => {
                // Handle loops if needed
            }
            _ => {}
        }
    }

    fn walk_contract(&mut self, contract: &ContractDefinition) {
        for node in &contract.nodes {
            self.walk_node(node, None, &contract.name, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: `visit_identifier` is called unconditionally by the walker, but
    // `visit_binary_op` only fires when a containing function is known (see
    // the `AstNode::BinaryOperation` arm above) - this test walks with no
    // function context, so it counts identifiers as the unconditional signal.
    struct CountingVisitor {
        identifiers_seen: usize,
    }

    impl AstVisitor for CountingVisitor {
        fn visit_identifier(&mut self, _id: &Identifier, _contract: &str) {
            self.identifiers_seen += 1;
        }
    }

    fn identifier_node(id: u64, name: &str) -> AstNode {
        AstNode::Identifier(Identifier {
            id,
            src: String::new(),
            name: name.to_string(),
            referenced_declaration: None,
            type_descriptions: None,
        })
    }

    /// Build a chain of `depth` nested `BinaryOperation` nodes, e.g. for
    /// depth=3: `((a + a) + a) + a` as a right-leaning AST.
    fn nested_binary_op_chain(depth: usize) -> AstNode {
        let mut node = identifier_node(0, "a");
        for i in 0..depth {
            node = AstNode::BinaryOperation(BinaryOperation {
                id: i as u64 + 1,
                src: String::new(),
                operator: "+".to_string(),
                left_expression: Box::new(node),
                right_expression: Box::new(identifier_node(1000 + i as u64, "a")),
                common_type: None,
            });
        }
        node
    }

    /// A contract with thousands of nested binary-op levels must not stack
    /// overflow the walker; nodes beyond MAX_WALK_DEPTH are simply skipped.
    #[test]
    fn walker_does_not_overflow_on_deeply_nested_expression() {
        let deeply_nested = nested_binary_op_chain(10_000);

        let mut visitor = CountingVisitor {
            identifiers_seen: 0,
        };
        {
            let mut walker = AstWalker::new(&mut visitor);
            walker.walk_node(&deeply_nested, None, "TestContract", 0);
        }

        // The walk must terminate (proving no stack overflow) and must have
        // stopped well short of visiting all ~10,001 identifiers - it should
        // be bounded by roughly MAX_WALK_DEPTH, not the full nesting depth.
        assert!(
            visitor.identifiers_seen < 10_000,
            "expected the depth guard to cut the walk short, but saw {} identifiers",
            visitor.identifiers_seen
        );
        assert!(visitor.identifiers_seen <= MAX_WALK_DEPTH * 2);
    }
}
