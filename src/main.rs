mod types;
mod parser;
use std::fs::File;
use std::io::prelude::*;

use parser::lexer::Lexer;
use clap::Parser;

use crate::types::elements;
use crate::types::elements::Renderable;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Markdown file path
    #[arg(short, long)]
    filepath: String 
}

fn main() {
    let args = Args::parse();
    let md = std::fs::read_to_string(args.filepath);
    
    let mut lexer = Lexer::new(md.unwrap().as_str());
    lexer.scan();
    let mut parser = parser::parser::Parser::new(lexer);
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
