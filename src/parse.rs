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

    pub fn next_token(&self) {
        unimplemented!()
    }
}
