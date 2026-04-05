//! Type definitions for Solidity AST from solc JSON output.
//!
//! These types model the AST structure produced by `solc --combined-json ast`.
//! They enable precise analysis of control flow, data flow, and vulnerability patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parse a "start:length:file" source location string
pub fn parse_src(src: &str) -> (u64, u64, u64) {
    let parts: Vec<&str> = src.split(':').collect();
    let start = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let length = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let file = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    (start, length, file)
}

/// Convert byte offset to line number in source text
pub fn offset_to_line(source: &str, byte_offset: u64) -> usize {
    source[..std::cmp::min(byte_offset as usize, source.len())]
        .chars()
        .filter(|&c| c == '\n')
        .count()
        + 1
}

/// Source unit (whole contract file)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceUnit {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// All nodes in this source unit
    pub nodes: Vec<AstNode>,
}

/// Contract definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractDefinition {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Contract name
    pub name: String,
    /// Kind: contract | interface | library
    #[serde(rename = "contractKind")]
    pub contract_kind: String,
    /// Base contracts
    #[serde(rename = "baseContracts")]
    pub base_contracts: Vec<InheritanceSpecifier>,
    /// All members (functions, state vars, etc.)
    pub nodes: Vec<AstNode>,
    /// Linearized base contracts in order
    #[serde(rename = "linearizedBaseContracts")]
    pub linearized_base_contracts: Vec<u64>,
}

/// Base contract reference
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InheritanceSpecifier {
    /// Base contract name
    #[serde(rename = "baseName")]
    pub base_name: Identifier,
}

/// Function definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionDefinition {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Function name
    pub name: String,
    /// Visibility: public | external | internal | private
    pub visibility: String,
    /// State mutability: pure | view | nonpayable | payable
    #[serde(rename = "stateMutability")]
    pub state_mutability: String,
    /// Is constructor
    #[serde(rename = "isConstructor")]
    pub is_constructor: bool,
    /// Applied modifiers
    pub modifiers: Vec<ModifierInvocation>,
    /// Parameters
    pub parameters: ParameterList,
    /// Return parameters
    #[serde(rename = "returnParameters")]
    pub return_parameters: ParameterList,
    /// Function body
    pub body: Option<Block>,
}

/// Modifier invocation in a function
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModifierInvocation {
    /// Modifier name
    #[serde(rename = "modifierName")]
    pub modifier_name: Identifier,
}

/// Parameter list
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParameterList {
    /// Parameters
    pub parameters: Vec<VariableDeclaration>,
}

/// Variable declaration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VariableDeclaration {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Variable name
    pub name: String,
    /// Is state variable
    #[serde(rename = "stateVariable")]
    pub state_variable: bool,
    /// Visibility
    pub visibility: String,
    /// Storage location: memory | storage | calldata
    #[serde(rename = "storageLocation")]
    pub storage_location: String,
    /// Type descriptions
    #[serde(rename = "typeName")]
    pub type_name: Option<Box<AstNode>>,
    /// Type information
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: Option<TypeDescription>,
}

/// Type description
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeDescription {
    /// Type identifier
    #[serde(rename = "typeIdentifier")]
    pub type_identifier: String,
    /// Type string representation
    #[serde(rename = "typeString")]
    pub type_string: String,
}

/// Code block (sequence of statements)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Statements in order
    pub statements: Vec<AstNode>,
}

/// Expression statement
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpressionStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// The expression
    pub expression: Option<Box<AstNode>>,
}

/// If statement
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IfStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Condition
    pub condition: Box<AstNode>,
    /// True branch
    #[serde(rename = "trueBody")]
    pub true_body: Box<AstNode>,
    /// False branch (if exists)
    #[serde(rename = "falseBody")]
    pub false_body: Option<Box<AstNode>>,
}

/// For loop
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
}

/// While loop
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhileStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
}

/// Return statement
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReturnStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
}

/// Assignment
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Assignment {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Left-hand side
    #[serde(rename = "leftHandSide")]
    pub left_hand_side: Box<AstNode>,
    /// Operator
    pub operator: String,
    /// Right-hand side
    #[serde(rename = "rightHandSide")]
    pub right_hand_side: Box<AstNode>,
}

/// Function call
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// The function expression
    pub expression: Box<AstNode>,
    /// Arguments
    pub arguments: Vec<AstNode>,
    /// Argument names (for named arguments)
    pub names: Vec<String>,
    /// Is try-call
    #[serde(rename = "tryCall")]
    pub try_call: bool,
}

/// Member access (e.g., obj.member)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberAccess {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// The object being accessed
    pub expression: Box<AstNode>,
    /// Member name
    #[serde(rename = "memberName")]
    pub member_name: String,
    /// Type descriptions
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: Option<TypeDescription>,
}

/// Array/mapping index access (e.g., arr[i])
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexAccess {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Base expression
    #[serde(rename = "baseExpression")]
    pub base_expression: Box<AstNode>,
    /// Index expression
    #[serde(rename = "indexExpression")]
    pub index_expression: Option<Box<AstNode>>,
}

/// Binary operation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BinaryOperation {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Operator
    pub operator: String,
    /// Left operand
    #[serde(rename = "leftExpression")]
    pub left_expression: Box<AstNode>,
    /// Right operand
    #[serde(rename = "rightExpression")]
    pub right_expression: Box<AstNode>,
    /// Common type
    #[serde(rename = "commonType")]
    pub common_type: Option<TypeDescription>,
}

/// Identifier
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Identifier {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Name
    pub name: String,
    /// Referenced declaration ID
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: Option<u64>,
    /// Type descriptions
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: Option<TypeDescription>,
}

/// Literal value
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Literal {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// The value
    pub value: String,
    /// Subdenomination (for numbers)
    pub subdenomination: Option<String>,
    /// Type descriptions
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: Option<TypeDescription>,
}

/// Emit statement
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmitStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Event call
    #[serde(rename = "eventCall")]
    pub event_call: Box<AstNode>,
}

/// Revert statement
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevertStatement {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Error call
    #[serde(rename = "errorCall")]
    pub error_call: Option<Box<AstNode>>,
}

/// Modifier definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModifierDefinition {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Modifier name
    pub name: String,
    /// Parameters
    pub parameters: ParameterList,
    /// Body
    pub body: Option<Block>,
}

/// State variable declaration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateVariableDeclaration {
    /// Node ID
    pub id: u64,
    /// Source location
    pub src: String,
    /// Variables
    pub variables: Vec<VariableDeclaration>,
}

/// The main AST node type — a discriminated union using serde tagging
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "nodeType")]
pub enum AstNode {
    /// Source unit
    #[serde(rename = "SourceUnit")]
    SourceUnit(SourceUnit),
    /// Contract definition
    #[serde(rename = "ContractDefinition")]
    ContractDefinition(ContractDefinition),
    /// Function definition
    #[serde(rename = "FunctionDefinition")]
    FunctionDefinition(FunctionDefinition),
    /// Modifier definition
    #[serde(rename = "ModifierDefinition")]
    ModifierDefinition(ModifierDefinition),
    /// State variable declaration
    #[serde(rename = "VariableDeclarationStatement")]
    StateVariableDeclaration(StateVariableDeclaration),
    /// Expression statement
    #[serde(rename = "ExpressionStatement")]
    ExpressionStatement(ExpressionStatement),
    /// If statement
    #[serde(rename = "IfStatement")]
    IfStatement(IfStatement),
    /// For statement
    #[serde(rename = "ForStatement")]
    ForStatement(ForStatement),
    /// While statement
    #[serde(rename = "WhileStatement")]
    WhileStatement(WhileStatement),
    /// Return statement
    #[serde(rename = "Return")]
    ReturnStatement(ReturnStatement),
    /// Assignment
    #[serde(rename = "Assignment")]
    Assignment(Assignment),
    /// Function call
    #[serde(rename = "FunctionCall")]
    FunctionCall(FunctionCall),
    /// Member access
    #[serde(rename = "MemberAccess")]
    MemberAccess(MemberAccess),
    /// Index access
    #[serde(rename = "IndexAccess")]
    IndexAccess(IndexAccess),
    /// Binary operation
    #[serde(rename = "BinaryOperation")]
    BinaryOperation(BinaryOperation),
    /// Identifier
    #[serde(rename = "Identifier")]
    Identifier(Identifier),
    /// Literal
    #[serde(rename = "Literal")]
    Literal(Literal),
    /// Block
    #[serde(rename = "Block")]
    Block(Block),
    /// Emit statement
    #[serde(rename = "EmitStatement")]
    EmitStatement(EmitStatement),
    /// Revert statement
    #[serde(rename = "RevertStatement")]
    RevertStatement(RevertStatement),
    /// Variable declaration (used in function params, etc.)
    #[serde(rename = "VariableDeclaration")]
    VariableDeclaration(VariableDeclaration),
    /// Catch other node types gracefully
    #[serde(other)]
    Other,
}

impl AstNode {
    /// Get type string if available
    pub fn get_type_string(&self) -> Option<String> {
        match self {
            AstNode::Identifier(id) => {
                id.type_descriptions.as_ref().map(|t| t.type_string.clone())
            }
            AstNode::Literal(lit) => {
                lit.type_descriptions.as_ref().map(|t| t.type_string.clone())
            }
            AstNode::MemberAccess(ma) => {
                ma.type_descriptions.as_ref().map(|t| t.type_string.clone())
            }
            AstNode::BinaryOperation(bo) => {
                bo.common_type.as_ref().map(|t| t.type_string.clone())
            }
            _ => None,
        }
    }
}
