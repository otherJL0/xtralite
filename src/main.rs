use rustyline::{Editor, Result};
use tree_sitter::{Language, Parser};

fn main() -> Result<()> {
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_sqlite() -> Language;
    }
    let language = unsafe { tree_sitter_sqlite() };
    parser.set_language(language).unwrap();
    let mut rl = Editor::<()>::new();
    loop {
        let line = rl.readline("xlite> ")?;
        let tree = parser.parse(&line, None).unwrap();
        let root_node = tree.root_node();
        println!("Line: {}", line); // eval / print
        println!("Root Node Kind: {}", root_node.kind()); // eval / print
    }
}
