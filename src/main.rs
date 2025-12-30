mod parser;
mod types;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use clap::Parser;
use parser::lexer::Lexer;

use crate::types::elements;
use crate::types::elements::Renderable;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Markdown file or directory path
    #[arg(short, long)]
    path: String,
}

fn convert_file(filepath: &Path) {
    if (!filepath.ends_with(".md")) {
        return;
    }
    let md = std::fs::read_to_string(filepath);

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
    let html_filepath = Path::new(filepath).with_extension("html");
    let mut file = File::create(html_filepath).unwrap();
    file.write_all(doc.as_bytes()).unwrap();
}

fn convert_dir(dir: &Path) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                convert_dir(&path);
            } else {
                convert_file(&path);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    if path.is_file() {
        convert_file(&path);
    } else if path.is_dir() {
        convert_dir(&path);
    }
}
