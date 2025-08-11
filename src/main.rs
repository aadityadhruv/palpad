mod types;
mod parser;
use std::fs::File;
use std::io::prelude::*;

use parser::lexer::Lexer;
use parser::parser::{Parser, AST};

use crate::types::elements;
use crate::types::elements::Renderable;

fn main() {
    // let doc = elements::HTML::new();
    // let doc = doc.render();
    // let mut file = File::create("out.html").unwrap();
    // file.write_all(doc.as_bytes()).unwrap();
    let md = "## Title\nText of a paragraph\n\n##### Second heading\nSome more text";
    let mut lexer = Lexer::new(md);
    lexer.scan();
    let mut parser = Parser::new(lexer);
    parser.parse();
    let root = parser.tree;
    let mut doc = elements::HTML::new();
    for child in root.children {
        let ast = child.item;
        doc.items.push(ast.convert_to_renderable());
    }
    let doc = doc.render();
    let mut file = File::create("out.html").unwrap();
    file.write_all(doc.as_bytes()).unwrap();
}
