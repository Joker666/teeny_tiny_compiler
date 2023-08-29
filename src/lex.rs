use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pub cur_pos: i32,
    pub cur_char: char,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut new_self = Self {
            source: format!("{}\n", source),
            // Source code to lex as a string.
            // Append a newline to simplify lexing/parsing
            cur_pos: -1,   // Current character in the string.
            cur_char: ' ', // Current position in the string.
        };
        new_self.next_char();
        new_self
    }

    /// Process the next character.
    pub fn next_char(&mut self) {
        self.cur_pos += 1;

        if self.cur_pos >= self.source.len() as i32 {
            self.cur_char = "\0".parse().unwrap();
        } else {
            self.cur_char = self.source.chars().nth(self.cur_pos as usize).unwrap();
        }
    }

    /// Return the lookahead character.
    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() as i32 {
            return '\0';
        }
        self.source.chars().nth((self.cur_pos + 1) as usize).unwrap()
    }

    /// Invalid token found, print error message and exit.
    pub fn abort(&self, message: &str) {
        panic!("{}", message)
    }

    /// Skip whitespace except newlines, which we will use to indicate the end of a statement.
    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' {
            self.next_char()
        }
    }

    /// Skip comments in the code.
    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char()
            }
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let mut token = Token::default();

        if self.cur_char == '+' {
            token = Token {
                text: String::from(self.cur_char),
                kind: TokenType::Plus,
            };
        } else if self.cur_char == '-' {
            token = Token {
                text: String::from(self.cur_char),
                kind: TokenType::Minus,
            };
        } else if self.cur_char == '*' {
            token = Token {
                text: String::from(self.cur_char),
                kind: TokenType::Asterisk,
            };
        } else if self.cur_char == '/' {
            token = Token {
                text: String::from(self.cur_char),
                kind: TokenType::Slash,
            };
        } else if self.cur_char == '\n' {
            token = Token {
                text: String::from(self.cur_char),
                kind: TokenType::Newline,
            };
        } else if self.cur_char == '\0' {
            token = Token {
                text: String::from(self.cur_char),
                kind: TokenType::Eof,
            };
        } else if self.cur_char == '=' {
            token = self.handle_next_char_for_composite_chars(TokenType::Eq, TokenType::EqEq);
        } else if self.cur_char == '>' {
            token = self.handle_next_char_for_composite_chars(TokenType::Gt, TokenType::GtEq);
        } else if self.cur_char == '<' {
            token = self.handle_next_char_for_composite_chars(TokenType::Lt, TokenType::LtEq);
        } else if self.cur_char == '!' {
            if self.peek() == '=' {
                let last_char = self.cur_char;
                self.next_char();
                token = Token {
                    text: format!("{}{}", last_char, self.cur_char),
                    kind: TokenType::NotEq,
                };
            } else {
                self.abort(&format!("Expected !=, got !{}", self.cur_char));
            }
        } else if self.cur_char == '"' {
            self.next_char();
            let start_pos = self.cur_pos;

            while self.cur_char != '"' {
                // Don't allow special characters in the string. No escape characters, newlines, tabs, or %.
                // We will be using C's printf on this string.
                match self.cur_char {
                    '\r' | '\n' | '\t' | '\\' | '%' => {
                        self.abort("Illegal character in string");
                    }
                    _ => {
                        self.next_char();
                    }
                }
            }

            let token_text = self.get_token_text(start_pos, self.cur_pos); // Get the substring.

            token = Token {
                text: token_text,
                kind: TokenType::String,
            };
        } else if self.cur_char.is_ascii_digit() {
            // Leading character is a digit, so this must be a number.
            // Get all consecutive digits and decimal if there is one.
            let start_pos = self.cur_pos;

            while self.peek().is_ascii_digit() {
                self.next_char();
            }

            if self.peek() == '.' {
                self.next_char();

                if !self.peek().is_ascii_digit() {
                    self.abort("Illegal character in number");
                }

                while self.peek().is_ascii_digit() {
                    self.next_char();
                }
            }

            let token_text = self.get_token_text(start_pos, self.cur_pos + 1);

            token = Token {
                text: token_text,
                kind: TokenType::Number,
            };
        } else if self.cur_char.is_alphabetic() {
            // Leading character is a letter, so this must be an identifier or a keyword.
            // Get all consecutive alphanumeric characters.

            let start_pos = self.cur_pos;

            while self.peek().is_alphabetic() {
                self.next_char();
            }

            // Check if the token is in the list of keywords.
            let token_text: String = self.get_token_text(start_pos, self.cur_pos + 1);
            let keyword = Token::check_if_keyword(&token_text);

            if keyword == TokenType::Unknown {
                token = Token {
                    text: token_text,
                    kind: TokenType::Ident,
                };
            } else {
                token = Token {
                    text: token_text,
                    kind: keyword,
                };
            }
        } else {
            self.abort(&format!("Unknown token: {}", self.cur_char));
        }
        self.next_char();
        token
    }

    fn handle_next_char_for_composite_chars(&mut self, token_type: TokenType, other_token_type: TokenType) -> Token {
        if self.peek() == '=' {
            let last_char = self.cur_char;
            self.next_char();
            Token {
                text: format!("{}{}", last_char, self.cur_char),
                kind: other_token_type,
            }
        } else {
            Token {
                text: String::from(self.cur_char),
                kind: token_type,
            }
        }
    }

    fn get_token_text(&self, start_pos: i32, end_pos: i32) -> String {
        self.source
            .chars()
            .skip(start_pos as usize)
            .take((end_pos - start_pos) as usize)
            .collect()
    }
}
