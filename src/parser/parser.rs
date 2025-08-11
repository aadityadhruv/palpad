use std::iter::Peekable;

use super::lexer::{TokenType, Token, Lexer};
use crate::types::elements;

struct Text {
    token: Token,
    text: String,
}
impl Into<elements::Paragraph> for &Text {
    fn into(self) -> elements::Paragraph {
        elements::Paragraph::new(self.text.clone())
    }
}

struct Heading {
    level: u8,
    text: Text,
}
impl Into<elements::Heading> for &Heading {
    fn into(self) -> elements::Heading {
        elements::Heading::new(self.text.text.clone(), self.level)
    }
}

struct Noop {}

pub trait AST {
    fn convert_to_renderable(&self) -> Box<dyn elements::Renderable>;
}

impl AST for Text {
    fn convert_to_renderable(&self) -> Box<dyn elements::Renderable> {
        let text: elements::Paragraph = self.into();
        return Box::new(text);
    }
}
impl AST for Heading {
    fn convert_to_renderable(&self) -> Box<dyn elements::Renderable> {
        let heading: elements::Heading = self.into();
        return Box::new(heading);
    }
}
impl AST for Noop {
    fn convert_to_renderable(&self) -> Box<dyn elements::Renderable> {
        let blank = elements::Paragraph::new(String::new());
        return Box::new(blank);
    }
}

pub struct Exp {
    pub item: Box<dyn AST>,
}

pub struct Node {
    pub children: Vec<Exp>,
}

pub struct Parser {
    lexer: Lexer,
    tokens: Peekable<std::vec::IntoIter<Token>>,
    pub tree: Node,
}


impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let root = Node { children: vec![] };
        let input_lexer = lexer;
        let input_tokens = input_lexer.tokens.clone().into_iter().peekable();
        Parser { lexer: input_lexer, tokens: input_tokens, tree: root }
    }

    /* Parse a Text block
     * TEXT
     */
    fn text(&mut self) -> Text {
        match &self.tokens.next() {
            Some(token) => {
                if token.token_type == TokenType::TEXT {
                    return Text { token: token.clone(), text: token.value.clone() }
                }
                panic!("Invalid expression for text!");
            },
            None => { panic!("Invalid expression for text!") }
        }
    }

    /* Parse a Heading
     * HASH heading | HASH text
     */
    fn heading(&mut self) -> Heading {
        let mut heading_size = 0;
        while self.tokens.peek().unwrap().token_type == TokenType::HASH {
            heading_size += 1;
            self.tokens.next();
        };
        let heading_text = self.text();
        return Heading { level: heading_size, text: heading_text }
    }

    /* exp
     * text | heading
     */
    fn exp(&mut self) -> Exp {
        let token = self.tokens.peek();
        if token.is_none() {
            return Exp {item: Box::new(Noop{}) };
        }
        let token = token.unwrap().clone();
        if token.token_type == TokenType::TEXT {
            let tree = self.text();
            return Exp { item: Box::new(tree) };
        }

        if token.token_type == TokenType::HASH {
            let tree = self.heading();
            return Exp { item: Box::new(tree) };
        }

        panic!("Invalid Exp type!")

    }

    /* Node
     * exp | exp NEWLINE node
     */
    fn node(&mut self) -> Node {
        let mut node = Node { children: vec![] };
        let exp = self.exp();
        node.children.push(exp);

        loop {
            match self.tokens.peek() {
                Some(token) => {
                if token.token_type == TokenType::NEWLINE { 
                    self.tokens.next();
                    continue
                } else {
                    node.children.push(self.exp());
                }
                },
                None => { break }
            }
        }
        return node;
    }

    pub fn parse(&mut self) {
        self.tree = self.node();
    }
}
