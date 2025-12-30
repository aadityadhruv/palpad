/*
 *
 * The Lexer.
 *
 * Read a file and generate a stream of Tokens, of type TokenType
 *
 */
use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    TEXT,
    NEWLINE,
    HASH,
    BACKTICK,
    ASTERISK,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    location: (u8, u8),
}

/*
 * A Lexer struct consists of a list of tokens, the input string which generated the list, and the
 * current position we are at in the lexing stage
 */
#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    input_string: String,
    position: (u8, u8), //Line, index
}

impl Lexer {
    // Read file contents, and construct the struct to get ready for lexing
    pub fn new(source: &str) -> Self {
        let string = source.to_string();
        Lexer {
            input_string: string.clone(),
            tokens: Vec::new(),
            position: (0, 0),
        }
    }

    // Main lexing loop. Read the input string to create a stream of tokens
    pub fn scan(&mut self) {
        let string = self.input_string.clone();
        let mut chars = string.chars().peekable();

        loop {
            let character = chars.next();
            match character {
                Some(c) => {
                    // What token are we currently reading?
                    self.scan_token(c, &mut chars);
                }
                None => break,
            }
        }
    }

    // The big switch case. Match the string to a particular token
    // Certain tokens/chars lead to extra actions, such as the new line char, which will modifiy
    // self.position
    fn scan_token(&mut self, token: char, iter: &mut Peekable<Chars>) {
        match token {
            // Headers
            '#' => {
                let token = Token {
                    token_type: TokenType::HASH,
                    value: token.to_string(),
                    location: self.position,
                };
                self.position.1 += 1;
                self.tokens.push(token);
            }

            // Asterisk - bold or italic statement
            '*' => {
                let token = Token {
                    token_type: TokenType::ASTERISK,
                    value: token.to_string(),
                    location: self.position,
                };
                self.position.1 += 1;
                self.tokens.push(token);
            }
            // Backtick - in line code
            '`' => {
                let token = Token {
                    token_type: TokenType::BACKTICK,
                    value: token.to_string(),
                    location: self.position,
                };
                self.position.1 += 1;
                self.tokens.push(token);
            }
            // New line
            '\n' => {
                let token = Token {
                    token_type: TokenType::NEWLINE,
                    value: token.to_string(),
                    location: self.position,
                };
                self.position.0 += 1;
                self.position.1 = 0;
                self.tokens.push(token);
            }

            // Just strings
            _ => {
                let mut text = String::from(token);
                let mut position = 1;
                let specials = vec!['\n', '*', '_', '`'];
                loop {
                    let peeked = iter.peek();
                    match peeked {
                        Some(peek) => {
                            if specials.contains(peek) {
                                break;
                            }
                            text.push(peek.to_owned());
                            position += 1;
                            iter.next();
                        }
                        None => {
                            break;
                        }
                    };
                }
                let token = Token {
                    token_type: TokenType::TEXT,
                    value: text,
                    location: self.position,
                };
                self.position.1 += position;
                self.tokens.push(token);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let source = "## This is a heading\nI am a bunch of paragraph text. I can get pretty long.";
        let mut scanner = Lexer::new(source);
        scanner.scan();
        let tokens = vec![
            Token {
                token_type: TokenType::HASH,
                value: "#".to_string(),
                location: (0, 0),
            },
            Token {
                token_type: TokenType::HASH,
                value: "#".to_string(),
                location: (0, 1),
            },
            Token {
                token_type: TokenType::TEXT,
                value: " This is a heading".to_string(),
                location: (0, 2),
            },
            Token {
                token_type: TokenType::NEWLINE,
                value: "\n".to_string(),
                location: (0, 20),
            },
            Token {
                token_type: TokenType::TEXT,
                value: "I am a bunch of paragraph text. I can get pretty long.".to_string(),
                location: (1, 0),
            },
        ];
        assert_eq!(tokens.len(), scanner.tokens.len());
        for (index, token) in scanner.tokens.iter().enumerate() {
            assert_eq!(tokens.get(index).unwrap(), token);
        }
    }
    #[test]
    fn italics() {
        let source = "I am *italics*";
        let mut scanner = Lexer::new(source);
        scanner.scan();
        let tokens = vec![
            Token {
                token_type: TokenType::TEXT,
                value: "I am ".to_string(),
                location: (0, 0),
            },
            Token {
                token_type: TokenType::ASTERISK,
                value: "*".to_string(),
                location: (0, 5),
            },
            Token {
                token_type: TokenType::TEXT,
                value: "italics".to_string(),
                location: (0, 6),
            },
            Token {
                token_type: TokenType::ASTERISK,
                value: "*".to_string(),
                location: (0, 13),
            },
        ];
        assert_eq!(tokens.len(), scanner.tokens.len());
        for (index, token) in scanner.tokens.iter().enumerate() {
            assert_eq!(tokens.get(index).unwrap(), token);
        }
    }

    #[test]
    fn bold() {
        let source = "I am **bold**";
        let mut scanner = Lexer::new(source);
        scanner.scan();
        let tokens = vec![
            Token {
                token_type: TokenType::TEXT,
                value: "I am ".to_string(),
                location: (0, 0),
            },
            Token {
                token_type: TokenType::ASTERISK,
                value: "*".to_string(),
                location: (0, 5),
            },
            Token {
                token_type: TokenType::ASTERISK,
                value: "*".to_string(),
                location: (0, 6),
            },
            Token {
                token_type: TokenType::TEXT,
                value: "bold".to_string(),
                location: (0, 7),
            },
            Token {
                token_type: TokenType::ASTERISK,
                value: "*".to_string(),
                location: (0, 11),
            },
            Token {
                token_type: TokenType::ASTERISK,
                value: "*".to_string(),
                location: (0, 12),
            },
        ];
        assert_eq!(tokens.len(), scanner.tokens.len());
        for (index, token) in scanner.tokens.iter().enumerate() {
            assert_eq!(tokens.get(index).unwrap(), token);
        }
    }

    #[test]
    fn code() {
        let source = "I am `code`";
        let mut scanner = Lexer::new(source);
        scanner.scan();
        let tokens = vec![
            Token {
                token_type: TokenType::TEXT,
                value: "I am ".to_string(),
                location: (0, 0),
            },
            Token {
                token_type: TokenType::BACKTICK,
                value: "`".to_string(),
                location: (0, 5),
            },
            Token {
                token_type: TokenType::TEXT,
                value: "code".to_string(),
                location: (0, 6),
            },
            Token {
                token_type: TokenType::BACKTICK,
                value: "`".to_string(),
                location: (0, 10),
            },
        ];
        assert_eq!(tokens.len(), scanner.tokens.len());
        for (index, token) in scanner.tokens.iter().enumerate() {
            assert_eq!(tokens.get(index).unwrap(), token);
        }
    }
}
