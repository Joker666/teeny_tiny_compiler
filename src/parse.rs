use super::lex::Lexer;
use super::token::Token;
use super::token::TokenType;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl Parser {
    /// Parser object keeps track of current token and checks if the code matches the grammar.
    pub fn new(lexer: Lexer) -> Self {
        let mut new_self = Self {
            lexer,
            cur_token: None,
            peek_token: None,
            symbols: Default::default(),
            labels_declared: Default::default(),
            labels_gotoed: Default::default(),
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
                self.abort(&format!("Expected {:?}, got {:?}", kind, cur_token.kind))
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
        println!("PROGRAM");

        while !self.check_token(TokenType::Eof) {
            self.statement();
        }
    }

    /// One of the following statements...
    pub fn statement(&mut self) {
        // Check the first token to see what kind of statement this is.
        // "PRINT" (expression | string)
        if self.check_token(TokenType::Print) {
            println!("STATEMENT-PRINT");
            self.next_token();

            if self.check_token(TokenType::String) {
                self.next_token();
            } else {
                self.expression();
            }
        } else if self.check_token(TokenType::If) {
            // Branched statement
            // "IF" comparison "THEN" {statement} "ENDIF"
            println!("STATEMENT-IF");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::Then);
            self.nl();

            while !self.check_token(TokenType::EndIf) {
                self.statement();
            }

            self.match_token(TokenType::EndIf);
        } else if self.check_token(TokenType::While) {
            // Branched statement
            // "WHILE" comparison "REPEAT" {statement} "ENDWHILE"
            println!("STATEMENT-WHILE");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::Repeat);
            self.nl();

            while !self.check_token(TokenType::EndWhile) {
                self.statement();
            }

            self.match_token(TokenType::EndWhile);
        } else if self.check_token(TokenType::Label) {
            // "LABEL" ident
            println!("STATEMENT-LABEL");
            self.next_token();

            // Make sure this label doesn't already exist.
            if let Some(cur_token) = &self.cur_token {
                if self.labels_declared.contains(&cur_token.text) {
                    self.abort(&format!("Label already exists: {}", cur_token.text))
                }
            }

            self.match_token(TokenType::Ident);
        } else if self.check_token(TokenType::Goto) {
            // "GOTO" ident
            println!("STATEMENT-GOTO");
            self.next_token();

            if let Some(cur_token) = &self.cur_token {
                self.labels_gotoed.insert(cur_token.text.clone());
            }

            self.match_token(TokenType::Ident);
        } else if self.check_token(TokenType::Let) {
            // "LET" ident "=" expression
            println!("STATEMENT-LET");
            self.next_token();

            if let Some(cur_token) = &self.cur_token {
                if !self.symbols.contains(&cur_token.text) {
                    self.symbols.insert(cur_token.text.clone());
                }
            }

            self.match_token(TokenType::Ident);
            self.match_token(TokenType::Eq);
            self.expression();
        }

        // Newline
        self.nl();
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
