use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

use common::token::Token;

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

    fn get_token(&self) -> Token {
        todo!()
    }

    fn lex_file(&self) {}

    fn lex_stdin(&self) {
        let mut line: String = String::new();
        loop {
            print!("Fragment REPL >> ");
            line.clear();
            let _ = stdout().flush();
            if stdin().read_line(&mut line).is_err() {
                continue;
            };
        }
    }

    pub fn lex(&self) {
        match self.current_file {
            Some(_) => self.lex_file(),
            None => self.lex_stdin(),
        }
    }
}
