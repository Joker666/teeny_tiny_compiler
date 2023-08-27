#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum TokenType {
    Unknown = -2,
    Eof = -1,
    Newline = 0,
    Number,
    Ident,
    String,
    // Keywords
    Label = 101,
    Goto,
    Print,
    PrintLn,
    Input,
    Let,
    If,
    Then,
    EndIf,
    While,
    Repeat,
    EndWhile,
    // Operators
    Eq = 201,
    Plus,
    Minus,
    Asterisk,
    Slash,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}

#[derive(Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    pub fn new(token_text: &str, kind: TokenType) -> Self {
        Self {
            text: token_text.parse().unwrap(),
            kind,
        }
    }

    pub fn check_if_keyword(token_text: &str) -> TokenType {
        match token_text {
            "LABEL" => TokenType::Label,
            "GOTO" => TokenType::Goto,
            "PRINT" => TokenType::Print,
            "PRINTLN" => TokenType::PrintLn,
            "INPUT" => TokenType::Input,
            "LET" => TokenType::Let,
            "IF" => TokenType::If,
            "THEN" => TokenType::Then,
            "ENDIF" => TokenType::EndIf,
            "WHILE" => TokenType::While,
            "REPEAT" => TokenType::Repeat,
            "ENDWHILE" => TokenType::EndWhile,
            _ => TokenType::Unknown,
        }
    }
}
