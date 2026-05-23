#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Reporting engine: Generate JSON, Markdown, and CLI reports.

pub mod formatter;
pub mod formatter_ansi;
pub mod report;

pub use formatter::ReportFormatter;
pub use formatter_ansi::{format_terminal, format_ndjson, format_sarif};
pub use report::Report;
