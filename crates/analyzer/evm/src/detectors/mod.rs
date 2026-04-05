//! AST-based vulnerability detectors for EVM smart contracts.
//!
//! Each detector implements analysis for a specific vulnerability pattern.

pub mod access_control;
pub mod flash_loan;
pub mod overflow;
pub mod reentrancy;

pub use access_control::AccessControlDetector;
pub use flash_loan::FlashLoanDetector;
pub use overflow::OverflowDetector;
pub use reentrancy::ReentrancyDetector;
