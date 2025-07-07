use std::{str::Chars, iter::Peekable};

#[derive(Debug, PartialEq)]
enum TokenType {
    TEXT,
    NEWLINE,
    HASH,
    BACKTICK,
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    value: String,
    location: (u8, u8),
}


#[derive(Debug)]
struct Scanner {
    tokens: Vec<Token>,
    input_string: String,
    position: (u8, u8), //Line, index
}

impl Scanner {
    fn new(source: &str) -> Self {
        let string = source.to_string();
        Scanner { input_string: string.clone(), tokens: Vec::new(), position: (0, 0) }
    }


    fn scan(&mut self) {
        let string = self.input_string.clone();
        let mut chars = string.chars().peekable();

        loop {
            let character = chars.next();
            match character {
                Some(c) => {
                    self.scan_token(c, &mut chars);
                },
                None => { break }
            }
        }
    }

    fn scan_token(&mut self, token: char, iter: &mut Peekable<Chars>) {
        match token {

            '#' => { 
                let token = Token { token_type: TokenType::HASH, value: token.to_string(), location: self.position }; 
                self.position.1 += 1;
                self.tokens.push(token);
            },

            '\n' => {  
                let token = Token { token_type: TokenType::NEWLINE, value: token.to_string(), location: self.position };
                self.position.0 += 1;
                self.position.1 = 0;
                self.tokens.push(token);
            },

            _ => { 
                let mut text = String::from(token);
                let mut position = 1;
                loop {
                    let peeked = iter.peek();
                    match peeked {
                        Some(peek) => {
                            if *peek == '\n' {
                                break;
                            }
                            text.push(peek.to_owned());
                            position += 1;
                            iter.next();
                        },
                        None => {break;}
                    };
                };
                let token = Token { token_type: TokenType::TEXT, value: text, location: self.position };
                self.position.1 += position;
                self.tokens.push(token);
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let source = "## This is a heading\nI am a bunch of paragraph text. I can get pretty long.";
        let mut scanner = Scanner::new(source);
        scanner.scan();
        let tokens = vec![
            Token { token_type: TokenType::HASH, value: "#".to_string(), location: (0, 0) },
            Token { token_type: TokenType::HASH, value: "#".to_string(), location: (0, 1) },
            Token { token_type: TokenType::TEXT, value: " This is a heading".to_string(), location: (0, 2) },
            Token { token_type: TokenType::NEWLINE, value: "\n".to_string(), location: (0, 20) },
            Token { token_type: TokenType::TEXT, value: "I am a bunch of paragraph text. I can get pretty long.".to_string(), location: (1, 0) },
        ];
        assert_eq!(tokens.len(), scanner.tokens.len());
        for (index, token) in scanner.tokens.iter().enumerate() {
            assert_eq!(tokens.get(index).unwrap(), token);
        }
    }
}
