use std::path::Path;

fn main() {
    let src_dir = Path::new("vendor/tree-sitter-move-sui/src");
    let parser_path = src_dir.join("parser.c");

    cc::Build::new()
        .include(src_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(&parser_path)
        .compile("tree-sitter-move-sui");

    println!("cargo:rerun-if-changed={}", parser_path.display());
}
