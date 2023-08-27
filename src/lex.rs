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

    // Process the next character.
    pub fn next_char(&mut self) {
        self.cur_pos += 1;

        if self.cur_pos >= self.source.len() as i32 {
            self.cur_char = "\0".parse().unwrap();
        } else {
            self.cur_char = self.source.chars().nth(self.cur_pos as usize).unwrap();
        }
    }

    // Return the lookahead character.
    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() as i32 {
            return '\0';
        }
        self.source.chars().nth((self.cur_pos + 1) as usize).unwrap()
    }

    // Invalid token found, print error message and exit.
    pub fn abort(message: String) {
        panic!("{}", message)
    }

    // Skip whitespace except newlines, which we will use to indicate the end of a statement.
    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' {
            self.next_char()
        }
    }

    // Skip comments in the code.
    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char()
            }
        }
    }

    pub fn get_token(&self) {}
}