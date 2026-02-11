//! Invariant DSL grammar definition using pest.

use pest_derive::Parser;

/// The Invar DSL grammar.
#[allow(missing_docs, non_camel_case_types)]
#[derive(Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " | "\t" | NEWLINE }
NEWLINE = @{ "\r\n" | "\n" }

// Identifiers and literals
identifier = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
integer = @{ "-"? ~ ASCII_DIGIT+ }

// Operators (ordered by precedence)
eq = { "==" }
neq = { "!=" }
lt = { "<" }
gt = { ">" }
lte = { "<=" }
gte = { ">=" }
and = { "&&" }
or = { "||" }
not = { "!" }

// Literals
boolean = @{ "true" | "false" }

// Function call - must be tried before identifier
function_call = { identifier ~ "(" ~ (expr ~ ("," ~ expr)*)? ~ ")" }

// Atoms: function calls, literals, or identifiers (in order of specificity)
atom = _{ function_call | boolean | integer | identifier }

// Primary expressions with parentheses
primary = { "(" ~ expr ~ ")" | atom }

// Unary operators
unary = { not* ~ primary }

// Comparison operators
comparison = { unary ~ ((eq | neq | lte | gte | lt | gt) ~ unary)* }

// Logical AND
logical_and = { comparison ~ (and ~ comparison)* }

// Logical OR  
logical_or = { logical_and ~ (or ~ logical_and)* }

// Main expression
expr = { logical_or }

// Top-level invariant
invariant_def = {
    "invariant" ~ identifier ~ "{" ~ expr ~ "}"
}

file = { SOI ~ invariant_def+ ~ EOI }
"#]
pub struct InvarGrammar;

pub use InvarGrammar as Grammar;
