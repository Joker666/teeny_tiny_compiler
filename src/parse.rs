use super::lex::Lexer;
use super::token::Token;

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let new_self = Self {
            lexer,
            cur_token: None,
            peek_token: None,
        };

        new_self.next_token();
        new_self.next_token();

        new_self
    }

    pub fn check_token(&self) {
        unimplemented!()
    }

    pub fn check_peek(&self) {
        unimplemented!()
    }

    pub fn match_token(&self) {
        unimplemented!()
    }

    pub fn next_token(&self) {
        unimplemented!()
    }

    pub fn abort(&self, message: &str) {
        panic!("{}", message)
    }

    pub fn nl(&self) {
        unimplemented!()
    }

    pub fn statement(&self) {
        unimplemented!()
    }

    pub fn program(&self) {
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
