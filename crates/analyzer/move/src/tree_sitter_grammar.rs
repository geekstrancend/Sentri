//! Rust binding for the vendored Sui Move tree-sitter grammar.
//!
//! See `vendor/tree-sitter-move-sui/PROVENANCE.md` for where this grammar
//! came from and why it's vendored rather than a live git dependency.

// The crate-wide `#![deny(unsafe_code)]` is overridden narrowly here: calling
// into the C parser compiled by build.rs inherently requires an `extern "C"`
// declaration and an `unsafe` call to invoke it. This is the standard,
// unavoidable shape of every tree-sitter language binding - the FFI surface
// is exactly one function, immediately wrapped in a safe `language()`.
#![allow(unsafe_code)]

extern "C" {
    fn tree_sitter_move() -> tree_sitter::Language;
}

/// The tree-sitter [`Language`](tree_sitter::Language) for Sui Move.
pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_move() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grammar_loads_into_a_parser() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language())
            .expect("the vendored grammar must load into a tree-sitter Parser");
    }

    #[test]
    fn parses_a_minimal_module_without_error() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language()).unwrap();

        let source = "module a::x;\n\npublic entry fun withdraw(amount: u64) {}\n";
        let tree = parser
            .parse(source, None)
            .expect("parser must return a tree");
        assert!(
            !tree.root_node().has_error(),
            "expected clean parse, got error node(s): {}",
            tree.root_node().to_sexp()
        );
    }

    #[test]
    fn parses_sui_specific_public_package_visibility() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language()).unwrap();

        // public(package) is Sui-specific (distinct from Aptos's public(friend))
        // and is exactly the kind of construct a generic/mislabeled grammar
        // would fail to recognize.
        let source = "module a::x;\n\npublic(package) fun internal_only() {}\n";
        let tree = parser
            .parse(source, None)
            .expect("parser must return a tree");
        assert!(
            !tree.root_node().has_error(),
            "expected clean parse of public(package), got error node(s): {}",
            tree.root_node().to_sexp()
        );
    }
}
