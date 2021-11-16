use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-sqlite", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-sqlite");
}
