use super::emitter::Emitter;
use super::lex::Lexer;
use super::token::Token;
use super::token::TokenType;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer,
    emitter: &'a mut Emitter,
    cur_token: Token,
    peek_token: Token,
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl<'a> Parser<'a> {
    /// Parser object keeps track of current token and checks if the code matches the grammar
    pub fn new(lexer: Lexer, emitter: &'a mut Emitter) -> Self {
        let mut new_self = Self {
            lexer,
            emitter,
            cur_token: Token::default(),
            peek_token: Token::default(),
            symbols: Default::default(),         // Variables declared so far
            labels_declared: Default::default(), // Labels declared so far
            labels_gotoed: Default::default(),   // Labels goto'ed so far
        };

        new_self.next_token();
        new_self.next_token(); // Call this twice to initialize current and peek

        new_self
    }

    /// Return true if the current token matches
    pub fn check_token(&self, kind: TokenType) -> bool {
        self.cur_token.kind == kind
    }

    /// Try to match current token. If matched advances the current token, If not, error.
    pub fn match_token(&mut self, kind: TokenType) {
        if self.cur_token.kind != kind {
            self.abort(&format!("Expected {:?}, got {:?}", kind, self.cur_token.kind))
        }

        self.next_token();
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
        // Require at least one newline
        self.match_token(TokenType::Newline);

        // But we will allow extra newlines too, of course
        while self.check_token(TokenType::Newline) {
            self.match_token(TokenType::Newline);
        }
    }

    /// program ::= {statement}
    pub fn program(&mut self) {
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main(void) {");

        // Since some newlines are required in our grammar, need to skip the excess
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }

        // Parse all the statements in the program
        while !self.check_token(TokenType::Eof) {
            self.statement();
        }

        // Wrap things up
        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");

        // Check that each label referenced in a GOTO is declared
        for x in self.labels_gotoed.iter() {
            if !self.labels_declared.contains(x) {
                self.abort(&format!("Label {} referenced but not declared", x))
            }
        }
    }

    /// One of the following statements...
    pub fn statement(&mut self) {
        // Check the first token to see what kind of statement this is.
        // "PRINT" (expression | string)
        if self.check_token(TokenType::Print) {
            self.next_token();

            if self.check_token(TokenType::String) {
                self.emitter
                    .emit_line(&format!("{}{}{}", "printf(\"", self.cur_token.text, "\\n\");"));
                self.next_token();
            } else {
                self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                self.expression();
                self.emitter.emit_line("));");
            }
        } else if self.check_token(TokenType::If) {
            // Branched statement
            // "IF" comparison "THEN" {statement} "ENDIF"
            self.next_token();
            self.emitter.emit("if(");
            self.comparison();

            self.match_token(TokenType::Then);
            self.nl();
            self.emitter.emit_line("){");

            while !self.check_token(TokenType::EndIf) {
                self.statement();
            }

            self.match_token(TokenType::EndIf);
        } else if self.check_token(TokenType::While) {
            // Branched statement
            // "WHILE" comparison "REPEAT" {statement} "ENDWHILE"
            self.next_token();
            self.emitter.emit("while(");
            self.comparison();

            self.match_token(TokenType::Repeat);
            self.nl();
            self.emitter.emit_line("){");

            while !self.check_token(TokenType::EndWhile) {
                self.statement();
            }

            self.match_token(TokenType::EndWhile);
            self.emitter.emit_line("}")
        } else if self.check_token(TokenType::Label) {
            // "LABEL" ident
            self.next_token();

            // Make sure this label doesn't already exist.
            if self.labels_declared.contains(&self.cur_token.text) {
                self.abort(&format!("Label already exists: {}", self.cur_token.text))
            }

            self.emitter.emit_line(&format!("{}:", self.cur_token.text));
            self.match_token(TokenType::Ident);
        } else if self.check_token(TokenType::Goto) {
            // "GOTO" ident
            self.next_token();

            self.labels_gotoed.insert(self.cur_token.text.clone());
            self.emitter.emit_line(&format!("goto {};", self.cur_token.text));
            self.match_token(TokenType::Ident);
        } else if self.check_token(TokenType::Let) {
            // "LET" ident "=" expression
            self.next_token();

            // If variable doesn't already exist, declare it
            if !self.symbols.contains(&self.cur_token.text) {
                self.symbols.insert(self.cur_token.text.clone());
                self.emitter.header_line(&format!("float {};", self.cur_token.text));
            }

            self.emitter.emit(&format!("{} = ", self.cur_token.text));
            self.match_token(TokenType::Ident);
            self.match_token(TokenType::Eq);
            self.expression();
            self.emitter.emit_line(";");
        } else if self.check_token(TokenType::Input) {
            // "INPUT" ident
            self.next_token();

            // If variable doesn't already exist, declare it
            if !self.symbols.contains(&self.cur_token.text) {
                self.symbols.insert(self.cur_token.text.clone());
                self.emitter.header_line(&format!("float {};", self.cur_token.text));
            }

            self.emitter.emit_line(&format!(
                "{}{}{}",
                "if (0 == scanf(\"%f\", &", self.cur_token.text, ")) {"
            ));
            self.emitter.emit_line(&format!("{} = 0;", self.cur_token.text));
            self.emitter.emit_line("scanf(\"%*s\");");
            self.emitter.emit_line("}");
            self.match_token(TokenType::Ident);
        } else {
            self.abort(&format!(
                "Invalid statement at {} ({:?})",
                self.cur_token.text, self.cur_token.kind
            ))
        }

        // Newline
        self.nl();
    }

    /// comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    pub fn comparison(&mut self) {
        self.expression();

        // Must be at least one comparison operator and another expression
        if self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression();
        } else {
            self.abort(&format!("Expected comparison operator at: {}", self.cur_token.text))
        }

        // Can have 0 or more comparison operator and expressions
        while self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression();
        }
    }

    /// expression ::= term {( "-" | "+" ) term}
    pub fn expression(&mut self) {
        self.term();

        //  Can have 0 or more +/- and expressions
        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.term();
        }
    }

    /// term ::= unary {( "/" | "*" ) unary}
    pub fn term(&mut self) {
        self.unary();

        // Can have 0 or more *// and expressions
        while self.check_token(TokenType::Slash) || self.check_token(TokenType::Asterisk) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.unary();
        }
    }

    /// unary ::= ["+" | "-"] primary
    pub fn unary(&mut self) {
        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        }
        self.primary()
    }

    /// primary ::= number | ident
    pub fn primary(&mut self) {
        if self.check_token(TokenType::Number) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        } else if self.check_token(TokenType::Ident) {
            if !self.symbols.contains(&self.cur_token.text) {
                self.abort(&format!(
                    "Referencing variable before assignment: {}",
                    self.cur_token.text
                ))
            }

            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        } else {
            // Error!
            self.abort(&format!("Unexpected token at {}", self.cur_token.text))
        }
    }

    fn is_comparison_operator(&self) -> bool {
        self.check_token(TokenType::Gt)
            || self.check_token(TokenType::GtEq)
            || self.check_token(TokenType::Lt)
            || self.check_token(TokenType::LtEq)
            || self.check_token(TokenType::EqEq)
            || self.check_token(TokenType::NotEq)
    }
}
