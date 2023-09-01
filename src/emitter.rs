use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Emitter {
    pub full_path: String,
    pub header: String,
    pub code: String,
}

impl Emitter {
    pub fn new(full_path: &str) -> Self {
        Self {
            full_path: full_path.to_owned(),
            header: "".to_owned(),
            code: "".to_owned(),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code = format!("{}{}", self.code, code)
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code = format!("{}{}\n", self.code, code)
    }

    pub fn header_line(&mut self, code: &str) {
        self.header = format!("{}{}\n", self.header, code)
    }

    pub fn write_file(&self) {
        let mut output_file = File::create(&self.full_path).unwrap();
        output_file.write_all(self.header.as_bytes()).unwrap();
        output_file.write_all(self.code.as_bytes()).unwrap();
    }
}
