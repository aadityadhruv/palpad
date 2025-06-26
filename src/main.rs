mod types;
mod parser;
use std::fs::File;
use std::io::prelude::*;

use crate::types::elements;
use crate::types::elements::Renderable;

fn main() {
    let doc = elements::HTML::new();
    let doc = doc.render();
    let mut file = File::create("out.html").unwrap();
    file.write_all(doc.as_bytes()).unwrap();
}
