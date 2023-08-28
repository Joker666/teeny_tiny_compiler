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
    pub fn abort(&self, message: String) {
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

    pub fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comment();

        let mut token = None;

        if self.cur_char == '+' {
            token = Some(Token {
                text: String::from(self.cur_char),
                kind: TokenType::Plus,
            });
        } else if self.cur_char == '-' {
            token = Some(Token {
                text: String::from(self.cur_char),
                kind: TokenType::Minus,
            });
        } else if self.cur_char == '*' {
            token = Some(Token {
                text: String::from(self.cur_char),
                kind: TokenType::Asterisk,
            });
        } else if self.cur_char == '/' {
            token = Some(Token {
                text: String::from(self.cur_char),
                kind: TokenType::Slash,
            });
        } else if self.cur_char == '\n' {
            token = Some(Token {
                text: String::from(self.cur_char),
                kind: TokenType::Newline,
            });
        } else if self.cur_char == '\0' {
            token = Some(Token {
                text: String::from(self.cur_char),
                kind: TokenType::Eof,
            });
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
                token = Some(Token {
                    text: format!("{}{}", last_char, self.cur_char),
                    kind: TokenType::NotEq,
                });
            } else {
                self.abort(format!("Expected !=, got !{}", self.cur_char));
            }
        } else if self.cur_char == '"' {
            self.next_char();
            let start_pos = self.cur_pos;

            while self.cur_char != '"' {
                // Don't allow special characters in the string. No escape characters, newlines, tabs, or %.
                // We will be using C's printf on this string.
                if self.cur_char == '\r'
                    || self.cur_char == '\n'
                    || self.cur_char == '\t'
                    || self.cur_char == '\\'
                    || self.cur_char == '%'
                {
                    self.abort(String::from("Illegal character in string."));
                }
                self.next_char();
            }

            let token_text = self
                .source
                .chars()
                .skip(start_pos as usize)
                .take(self.cur_pos as usize)
                .collect(); // Get the substring.

            token = Some(Token {
                text: token_text,
                kind: TokenType::String,
            });
        } else if self.cur_char.is_digit(10) {
            // Leading character is a digit, so this must be a number.
            // Get all consecutive digits and decimal if there is one.
            let start_pos = self.cur_pos;

            while self.peek().is_digit(10) {
                self.next_char();
            }

            if self.peek() == '.' {
                self.next_char();

                if !self.peek().is_digit(10) {
                    self.abort(String::from("Illegal character in number."));
                }

                while self.peek().is_digit(10) {
                    self.next_char();
                }
            }

            let token_text = self
                .source
                .chars()
                .skip(start_pos as usize)
                .take((self.cur_pos + 1) as usize)
                .collect(); // Get the substring.

            token = Some(Token {
                text: token_text,
                kind: TokenType::Number,
            });
        } else {
            self.abort(format!("Unknown token: {}", self.cur_char));
        }
        self.next_char();
        token
    }

    fn handle_next_char_for_composite_chars(
        &mut self,
        token_type: TokenType,
        other_token_type: TokenType,
    ) -> Option<Token> {
        if self.peek() == '=' {
            let last_char = self.cur_char;
            self.next_char();
            Some(Token {
                text: format!("{}{}", last_char, self.cur_char),
                kind: other_token_type,
            })
        } else {
            Some(Token {
                text: String::from(self.cur_char),
                kind: token_type,
            })
        }
    }
}
