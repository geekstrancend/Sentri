#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Reporting engine: Generate JSON, Markdown, and CLI reports.

pub mod formatter;
pub mod report;

pub use report::Report;
pub use formatter::ReportFormatter;
