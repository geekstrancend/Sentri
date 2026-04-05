//! AST-based vulnerability detectors for EVM smart contracts.
//!
//! Each detector implements analysis for a specific vulnerability pattern.

pub mod reentrancy;
pub mod overflow;
pub mod access_control;
pub mod flash_loan;

pub use reentrancy::ReentrancyDetector;
pub use overflow::OverflowDetector;
pub use access_control::AccessControlDetector;
pub use flash_loan::FlashLoanDetector;
