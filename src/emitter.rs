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
}
