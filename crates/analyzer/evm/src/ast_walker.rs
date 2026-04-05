//! AST visitor pattern for walking and analyzing Solidity AST.
//!
//! The walker uses the visitor pattern to enable custom analyses on the AST.
//! Implementations can override specific visit/leave methods for the node types
//! they care about.

use crate::ast_types::*;

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
            self.walk_node(node, None, "");
        }
    }

    fn walk_node(
        &mut self,
        node: &AstNode,
        current_func: Option<&FunctionDefinition>,
        current_contract: &str,
    ) {
        match node {
            AstNode::ContractDefinition(contract) => {
                self.walk_contract(contract);
            }
            AstNode::FunctionDefinition(func) => {
                self.visitor.visit_function(func, current_contract);
                if let Some(body) = &func.body {
                    for stmt in &body.statements {
                        self.walk_node(stmt, Some(func), current_contract);
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
                    self.walk_node(arg, current_func, current_contract);
                }
                self.walk_node(&call.expression, current_func, current_contract);
            }
            AstNode::Assignment(assign) => {
                if let Some(func) = current_func {
                    self.visitor
                        .visit_assignment(assign, func, current_contract);
                }
                self.walk_node(&assign.left_hand_side, current_func, current_contract);
                self.walk_node(&assign.right_hand_side, current_func, current_contract);
            }
            AstNode::BinaryOperation(op) => {
                if let Some(func) = current_func {
                    self.visitor.visit_binary_op(op, func, current_contract);
                }
                self.walk_node(&op.left_expression, current_func, current_contract);
                self.walk_node(&op.right_expression, current_func, current_contract);
            }
            AstNode::Block(block) => {
                for stmt in &block.statements {
                    self.walk_node(stmt, current_func, current_contract);
                }
            }
            AstNode::ExpressionStatement(expr) => {
                if let Some(expression) = &expr.expression {
                    self.walk_node(expression, current_func, current_contract);
                }
            }
            AstNode::IfStatement(if_stmt) => {
                self.walk_node(&if_stmt.condition, current_func, current_contract);
                self.walk_node(&if_stmt.true_body, current_func, current_contract);
                if let Some(false_body) = &if_stmt.false_body {
                    self.walk_node(false_body, current_func, current_contract);
                }
            }
            AstNode::StateVariableDeclaration(decl) => {
                for var in &decl.variables {
                    self.visitor.visit_state_variable(var, current_contract);
                }
            }
            AstNode::MemberAccess(access) => {
                self.visitor.visit_member_access(access, current_contract);
                self.walk_node(&access.expression, current_func, current_contract);
            }
            AstNode::Identifier(id) => {
                self.visitor.visit_identifier(id, current_contract);
            }
            AstNode::IndexAccess(idx) => {
                self.walk_node(&idx.base_expression, current_func, current_contract);
                if let Some(index) = &idx.index_expression {
                    self.walk_node(index, current_func, current_contract);
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
            self.walk_node(node, None, &contract.name);
        }
    }
}
