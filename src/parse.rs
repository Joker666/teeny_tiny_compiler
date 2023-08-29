use super::lex::Lexer;
use super::token::Token;
use super::token::TokenType;

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    /// Parser object keeps track of current token and checks if the code matches the grammar.
    pub fn new(lexer: Lexer) -> Self {
        let mut new_self = Self {
            lexer,
            cur_token: None,
            peek_token: None,
        };

        new_self.next_token();
        new_self.next_token(); // Call this twice to initialize current and peek.

        new_self
    }

    /// Return true if the current token matches
    pub fn check_token(&self, kind: TokenType) -> bool {
        if let Some(cur_token) = &self.cur_token {
            return cur_token.kind == kind;
        }
        false
    }

    /// Return true if the next token matches
    pub fn check_peek(&self, kind: TokenType) -> bool {
        if let Some(peek_token) = &self.peek_token {
            return peek_token.kind == kind;
        }
        false
    }

    /// Try to match current token. If matched advances the current token, If not, error.
    pub fn match_token(&mut self, kind: TokenType) {
        if let Some(cur_token) = &self.cur_token {
            if cur_token.kind != kind {
                self.abort(format!("Expected {:?}, got {:?}", kind, cur_token.kind).as_str())
            }

            self.next_token();
        }
    }

    /// Advances the current token
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    pub fn abort(&self, message: &str) {
        panic!("{}", message)
    }

    // ////////////////////////
    // Lexer Interface
    // ////////////////////////

    /// nl ::= '\n'+
    pub fn nl(&mut self) {
        println!("NEWLINE");

        // Require at least one newline
        self.match_token(TokenType::Newline);

        // But we will allow extra newlines too, of course
        while self.check_token(TokenType::Newline) {
            self.match_token(TokenType::Newline);
        }
    }

    /// program ::= {statement}
    pub fn program(&mut self) {
        println!("Program");

        while self.check_token(TokenType::Eof) {
            self.statement();
        }
    }

    /// One of the following statements...
    pub fn statement(&self) {
        unimplemented!()
    }

    pub fn comparison(&self) {
        unimplemented!()
    }

    pub fn expression(&self) {
        unimplemented!()
    }

    pub fn term(&self) {
        unimplemented!()
    }

    pub fn unary(&self) {
        unimplemented!()
    }

    pub fn primary(&self) {
        unimplemented!()
    }

    fn is_comparison_operator(&self) {
        unimplemented!()
    }
}
