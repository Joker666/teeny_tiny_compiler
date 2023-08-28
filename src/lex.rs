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

        match self.cur_char {
            '+' => {
                token = Some(Token {
                    text: String::from(self.cur_char),
                    kind: TokenType::Plus,
                })
            }
            '-' => {
                token = Some(Token {
                    text: String::from(self.cur_char),
                    kind: TokenType::Minus,
                })
            }
            '*' => {
                token = Some(Token {
                    text: String::from(self.cur_char),
                    kind: TokenType::Asterisk,
                })
            }
            '/' => {
                token = Some(Token {
                    text: String::from(self.cur_char),
                    kind: TokenType::Slash,
                })
            }
            '\n' => {
                token = Some(Token {
                    text: String::from(self.cur_char),
                    kind: TokenType::Newline,
                })
            }
            '\0' => {
                token = Some(Token {
                    text: String::from(self.cur_char),
                    kind: TokenType::Eof,
                })
            }
            '=' => {
                token = self.handle_next_char(self.cur_char, TokenType::Eq);
            }
            '>' => {
                token = self.handle_next_char(self.cur_char, TokenType::Gt);
            }
            _ => self.abort(format!("Unknown token: {}", self.cur_char)),
        }

        token
    }

    fn handle_next_char(&mut self, last_char: char, token_type: TokenType) -> Option<Token> {
        if self.peek() == '=' {
            self.next_char();
            Some(Token {
                text: format!("{}{}", last_char, self.cur_char),
                kind: token_type,
            })
        } else {
            Some(Token {
                text: String::from(self.cur_char),
                kind: token_type,
            })
        }
    }
}
