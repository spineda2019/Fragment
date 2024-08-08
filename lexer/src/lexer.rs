use std::fs::File;

pub struct Lexer {
    current_file: Option<File>,
}

impl Lexer {
    pub fn init() -> Lexer {
        Lexer { current_file: None }
    }

    pub fn new_file(&mut self, file: File) {
        self.current_file = Some(file);
    }

    fn lex_file(&self) {}

    fn lex_stdin(&self) {}

    pub fn lex(&self) {
        match self.current_file {
            Some(_) => self.lex_file(),
            None => self.lex_stdin(),
        }
    }
}
