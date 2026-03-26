#![allow(missing_docs)]
//! EVM Bytecode analysis and disassembly.
//!
//! Analyzes compiled EVM bytecode to detect:
//! - Low-level vulnerabilities
//! - Opcode patterns
//! - Assembly-level issues
//! - Gas optimization opportunities

use crate::errors::{AnalysisError, AnalysisResult};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// EVM opcode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Opcode {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Sdiv,
    Mod,
    Smod,
    Addmod,
    Mulmod,
    Exp,
    SignExtend,

    // Comparison
    Lt,
    Gt,
    Slt,
    Sgt,
    Eq,
    IsZero,
    And,
    Or,
    Xor,
    Not,
    Byte,
    Shl,
    Shr,
    Sar,

    // Cryptographic
    Keccak256,

    // Stack
    Pop,
    Mload,
    Mstore,
    Mstore8,
    Sload,
    Sstore,
    Msize,

    // Flow
    Pc,
    Gas,
    Jump,
    Jumpi,
    Jumpdest,
    Return,
    Revert,
    Selfdestruct,

    // Calls
    Call,
    Callcode,
    Delegatecall,
    Staticcall,
    Create,
    Create2,

    // Data
    Calldataload,
    Calldatasize,
    Calldatacopy,
    Codesize,
    Codecopy,
    Extcodesize,
    Extcodecopy,
    Extcodehash,
    Returndatasize,
    Returndatacopy,

    // Environment
    Address,
    Balance,
    Origin,
    Caller,
    Callvalue,
    Gasprice,
    Blockcoinhash,
    Coinbase,
    Timestamp,
    Number,
    Difficulty,
    Gaslimit,
    Chainid,
    Selfbalance,

    // Push/Dup
    Push(u8),
    Dup(u8),
    Swap(u8),
    Log(u8),

    // Special
    Nop,
    Invalid,
    Unknown(u8),
}

impl Opcode {
    /// Parse opcode from byte.
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Nop,
            0x01 => Opcode::Add,
            0x02 => Opcode::Mul,
            0x03 => Opcode::Sub,
            0x04 => Opcode::Div,
            0x05 => Opcode::Sdiv,
            0x06 => Opcode::Mod,
            0x07 => Opcode::Smod,
            0x08 => Opcode::Addmod,
            0x09 => Opcode::Mulmod,
            0x0a => Opcode::Exp,
            0x0b => Opcode::SignExtend,
            0x10 => Opcode::Lt,
            0x11 => Opcode::Gt,
            0x12 => Opcode::Slt,
            0x13 => Opcode::Sgt,
            0x14 => Opcode::Eq,
            0x15 => Opcode::IsZero,
            0x16 => Opcode::And,
            0x17 => Opcode::Or,
            0x18 => Opcode::Xor,
            0x19 => Opcode::Not,
            0x1a => Opcode::Byte,
            0x1b => Opcode::Shl,
            0x1c => Opcode::Shr,
            0x1d => Opcode::Sar,
            0x20 => Opcode::Keccak256,
            0x50 => Opcode::Pop,
            0x51 => Opcode::Mload,
            0x52 => Opcode::Mstore,
            0x53 => Opcode::Mstore8,
            0x54 => Opcode::Sload,
            0x55 => Opcode::Sstore,
            0x56 => Opcode::Jump,
            0x57 => Opcode::Jumpi,
            0x58 => Opcode::Pc,
            0x59 => Opcode::Msize,
            0x5a => Opcode::Gas,
            0x5b => Opcode::Jumpdest,
            0xf0 => Opcode::Create,
            0xf1 => Opcode::Call,
            0xf2 => Opcode::Callcode,
            0xf3 => Opcode::Return,
            0xf4 => Opcode::Delegatecall,
            0xf5 => Opcode::Create2,
            0xfa => Opcode::Staticcall,
            0xfd => Opcode::Revert,
            0xfe => Opcode::Invalid,
            0xff => Opcode::Selfdestruct,
            b if (0x60..=0x7f).contains(&b) => Opcode::Push(b - 0x60 + 1),
            b if (0x80..=0x8f).contains(&b) => Opcode::Dup(b - 0x80 + 1),
            b if (0x90..=0x9f).contains(&b) => Opcode::Swap(b - 0x90 + 1),
            b if (0xa0..=0xa4).contains(&b) => Opcode::Log(b - 0xa0),
            _ => Opcode::Unknown(byte),
        }
    }

    /// Get opcode name.
    pub fn name(&self) -> &'static str {
        match self {
            Opcode::Add => "ADD",
            Opcode::Sub => "SUB",
            Opcode::Mul => "MUL",
            Opcode::Div => "DIV",
            Opcode::Sdiv => "SDIV",
            Opcode::Mod => "MOD",
            Opcode::Smod => "SMOD",
            Opcode::Addmod => "ADDMOD",
            Opcode::Mulmod => "MULMOD",
            Opcode::Exp => "EXP",
            Opcode::SignExtend => "SIGNEXTEND",
            Opcode::Lt => "LT",
            Opcode::Gt => "GT",
            Opcode::Slt => "SLT",
            Opcode::Sgt => "SGT",
            Opcode::Eq => "EQ",
            Opcode::IsZero => "ISZERO",
            Opcode::And => "AND",
            Opcode::Or => "OR",
            Opcode::Xor => "XOR",
            Opcode::Not => "NOT",
            Opcode::Byte => "BYTE",
            Opcode::Shl => "SHL",
            Opcode::Shr => "SHR",
            Opcode::Sar => "SAR",
            Opcode::Keccak256 => "KECCAK256",
            Opcode::Pop => "POP",
            Opcode::Mload => "MLOAD",
            Opcode::Mstore => "MSTORE",
            Opcode::Mstore8 => "MSTORE8",
            Opcode::Sload => "SLOAD",
            Opcode::Sstore => "SSTORE",
            Opcode::Pc => "PC",
            Opcode::Msize => "MSIZE",
            Opcode::Gas => "GAS",
            Opcode::Jump => "JUMP",
            Opcode::Jumpi => "JUMPI",
            Opcode::Jumpdest => "JUMPDEST",
            Opcode::Return => "RETURN",
            Opcode::Revert => "REVERT",
            Opcode::Selfdestruct => "SELFDESTRUCT",
            Opcode::Call => "CALL",
            Opcode::Callcode => "CALLCODE",
            Opcode::Delegatecall => "DELEGATECALL",
            Opcode::Staticcall => "STATICCALL",
            Opcode::Create => "CREATE",
            Opcode::Create2 => "CREATE2",
            Opcode::Calldataload => "CALLDATALOAD",
            Opcode::Calldatasize => "CALLDATASIZE",
            Opcode::Calldatacopy => "CALLDATACOPY",
            Opcode::Codesize => "CODESIZE",
            Opcode::Codecopy => "CODECOPY",
            Opcode::Extcodesize => "EXTCODESIZE",
            Opcode::Extcodecopy => "EXTCODECOPY",
            Opcode::Extcodehash => "EXTCODEHASH",
            Opcode::Returndatasize => "RETURNDATASIZE",
            Opcode::Returndatacopy => "RETURNDATACOPY",
            Opcode::Address => "ADDRESS",
            Opcode::Balance => "BALANCE",
            Opcode::Origin => "ORIGIN",
            Opcode::Caller => "CALLER",
            Opcode::Callvalue => "CALLVALUE",
            Opcode::Gasprice => "GASPRICE",
            Opcode::Blockcoinhash => "BLOCKHASH",
            Opcode::Coinbase => "COINBASE",
            Opcode::Timestamp => "TIMESTAMP",
            Opcode::Number => "NUMBER",
            Opcode::Difficulty => "DIFFICULTY",
            Opcode::Gaslimit => "GASLIMIT",
            Opcode::Chainid => "CHAINID",
            Opcode::Selfbalance => "SELFBALANCE",
            Opcode::Push(_) => "PUSH",
            Opcode::Dup(_) => "DUP",
            Opcode::Swap(_) => "SWAP",
            Opcode::Log(_) => "LOG",
            Opcode::Nop => "NOP",
            Opcode::Invalid => "INVALID",
            Opcode::Unknown(_) => "UNKNOWN",
        }
    }

    /// Check if opcode is a jump.
    pub fn is_jump(&self) -> bool {
        matches!(self, Opcode::Jump | Opcode::Jumpi)
    }

    /// Check if opcode is a call.
    pub fn is_call(&self) -> bool {
        matches!(
            self,
            Opcode::Call
                | Opcode::Callcode
                | Opcode::Delegatecall
                | Opcode::Staticcall
                | Opcode::Create
                | Opcode::Create2
        )
    }

    /// Check if opcode mutates state.
    pub fn mutates_state(&self) -> bool {
        matches!(self, Opcode::Sstore | Opcode::Create | Opcode::Create2)
    }
}

/// Bytecode instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    /// Program counter (offset in bytecode).
    pub pc: usize,
    /// Opcode.
    pub opcode: Opcode,
    /// Immediates (for PUSH instructions).
    pub immediates: Vec<u8>,
}

impl Instruction {
    /// Create instruction.
    pub fn new(pc: usize, opcode: Opcode, immediates: Vec<u8>) -> Self {
        Self {
            pc,
            opcode,
            immediates,
        }
    }

    /// Disassemble to string.
    pub fn disassemble(&self) -> String {
        match &self.opcode {
            Opcode::Push(_) => {
                let value = self
                    .immediates
                    .iter()
                    .fold(String::new(), |acc, b| format!("{}{:02x}", acc, b));
                format!("PUSH {} 0x{}", self.immediates.len(), value)
            }
            Opcode::Dup(n) => format!("DUP{}", n),
            Opcode::Swap(n) => format!("SWAP{}", n),
            Opcode::Log(n) => format!("LOG{}", n),
            _ => self.opcode.name().to_string(),
        }
    }
}

/// Bytecode disassembler and analyzer.
pub struct BytecodeAnalyzer;

impl BytecodeAnalyzer {
    /// Disassemble bytecode (hex string) to instructions.
    pub fn disassemble(bytecode: &str) -> AnalysisResult<Vec<Instruction>> {
        debug!("Disassembling bytecode ({} chars)", bytecode.len());

        // Remove 0x prefix if present
        let bytecode = bytecode.strip_prefix("0x").unwrap_or(bytecode);

        // Parse hex string to bytes
        let bytes = Self::hex_to_bytes(bytecode)
            .map_err(|e| AnalysisError::bytecode(format!("Invalid bytecode hex: {}", e)))?;

        let mut instructions = Vec::new();
        let mut pc = 0;

        while pc < bytes.len() {
            let byte = bytes[pc];
            let opcode = Opcode::from_byte(byte);

            let mut immediates = Vec::new();
            let mut next_pc = pc + 1;

            // Handle PUSH instructions
            if let Opcode::Push(n) = opcode {
                let push_size = n as usize;
                if pc + 1 + push_size <= bytes.len() {
                    immediates = bytes[pc + 1..pc + 1 + push_size].to_vec();
                    next_pc = pc + 1 + push_size;
                }
            }

            instructions.push(Instruction::new(pc, opcode, immediates));
            pc = next_pc;
        }

        info!("Disassembled {} instructions", instructions.len());
        Ok(instructions)
    }

    /// Convert hex string to bytes.
    fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("Odd-length hex string".to_string());
        }

        (0..hex.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex[i..i + 2], 16)
                    .map_err(|_| format!("Invalid hex: {}", &hex[i..i + 2]))
            })
            .collect()
    }

    /// Analyze bytecode for vulnerabilities.
    pub fn analyze(bytecode: &str) -> AnalysisResult<BytecodeAnalysis> {
        let instructions = Self::disassemble(bytecode)?;

        let mut analysis = BytecodeAnalysis::new();

        for (i, instr) in instructions.iter().enumerate() {
            // Track calls
            if instr.opcode.is_call() {
                analysis.calls.push(instr.pc);
            }

            // Track jumps
            if instr.opcode.is_jump() {
                analysis.jumps.push(instr.pc);
            }

            // Track state mutations
            if instr.opcode.mutates_state() {
                analysis.state_mutations.push(instr.pc);
            }

            // Detect unsafe call patterns
            if i > 0 && instr.opcode.is_call() {
                let prev = &instructions[i - 1];
                if !matches!(prev.opcode, Opcode::Gas) {
                    // Missing gas specification
                    analysis.issues.push(BytecodeIssue {
                        severity: Severity::Medium,
                        issue_type: IssueType::UnsafeCall,
                        pc: instr.pc,
                        description: "Call without explicit gas limit".to_string(),
                    });
                }
            }

            // Detect reentrancy patterns
            if instr.opcode.is_call() && i > 0 {
                // Look for state mutations after call
                for future in instructions.iter().skip(i + 1).take(10) {
                    if future.opcode.mutates_state() {
                        analysis.issues.push(BytecodeIssue {
                            severity: Severity::High,
                            issue_type: IssueType::PotentialReentrancy,
                            pc: instr.pc,
                            description: "State mutation after call (reentrancy risk)".to_string(),
                        });
                        break;
                    }
                }
            }
        }

        analysis.instruction_count = instructions.len();
        Ok(analysis)
    }
}

/// Result of bytecode analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeAnalysis {
    /// Total instructions.
    pub instruction_count: usize,
    /// Detected call locations (PC offsets).
    pub calls: Vec<usize>,
    /// Detected jump locations.
    pub jumps: Vec<usize>,
    /// Detected state mutations.
    pub state_mutations: Vec<usize>,
    /// Identified issues.
    pub issues: Vec<BytecodeIssue>,
}

impl BytecodeAnalysis {
    /// Create empty analysis.
    pub fn new() -> Self {
        Self {
            instruction_count: 0,
            calls: Vec::new(),
            jumps: Vec::new(),
            state_mutations: Vec::new(),
            issues: Vec::new(),
        }
    }

    /// Check if analysis found critical issues.
    pub fn has_critical(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == Severity::Critical)
    }
}

impl Default for BytecodeAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

/// Issue found in bytecode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeIssue {
    /// Issue severity.
    pub severity: Severity,
    /// Issue type.
    pub issue_type: IssueType,
    /// Program counter where issue occurs.
    pub pc: usize,
    /// Human-readable description.
    pub description: String,
}

/// Issue severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Critical issue.
    Critical,
    /// High severity.
    High,
    /// Medium severity.
    Medium,
    /// Low severity.
    Low,
    /// Informational.
    Info,
}

/// Issue types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    /// Reentrancy vulnerability.
    PotentialReentrancy,
    /// Unsafe external call.
    UnsafeCall,
    /// Integer overflow.
    IntegerOverflow,
    /// Delegatecall to untrusted code.
    DangerousDelegatecall,
    /// Unprotected function.
    MissingAccessControl,
    /// Other issue.
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_parsing() {
        assert_eq!(Opcode::from_byte(0x01), Opcode::Add);
        assert_eq!(Opcode::from_byte(0x02), Opcode::Mul);
        assert_eq!(Opcode::from_byte(0xf1), Opcode::Call);
        assert_eq!(Opcode::from_byte(0xff), Opcode::Selfdestruct);
    }

    #[test]
    fn test_opcode_properties() {
        assert!(Opcode::Call.is_call());
        assert!(!Opcode::Add.is_call());
        assert!(Opcode::Jump.is_jump());
        assert!(Opcode::Sstore.mutates_state());
    }

    #[test]
    fn test_hex_to_bytes() {
        let bytes = BytecodeAnalyzer::hex_to_bytes("0102ff").unwrap();
        assert_eq!(bytes, vec![0x01, 0x02, 0xff]);

        let odd_hex = BytecodeAnalyzer::hex_to_bytes("010");
        assert!(odd_hex.is_err());
    }

    #[test]
    fn test_bytecode_analysis_creation() {
        let analysis = BytecodeAnalysis::new();
        assert_eq!(analysis.instruction_count, 0);
        assert!(!analysis.has_critical());
    }
}
