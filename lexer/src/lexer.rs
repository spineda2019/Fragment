use std::{
    fs::File,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use common::{error::CompilerError, token::Token};

use crate::char_reader::CharReader;

pub struct Lexer {
    current_file: Option<PathBuf>,
    current_line_number: usize,
}

impl Lexer {
    pub fn init() -> Lexer {
        Lexer {
            current_file: None,
            current_line_number: 0,
        }
    }

    pub fn new_file(&mut self, file: PathBuf) -> Result<(), CompilerError> {
        match file.extension() {
            None => return Err(CompilerError::UnrecognizedFileError(file)),
            Some(extension) => {
                match extension.to_str() {
                    Some("fr") => {}
                    _ => return Err(CompilerError::UnrecognizedFileError(file)),
                };
            }
        };

        self.current_file = Some(file);
        self.current_line_number = 0;
        Ok(())
    }

    fn lex_file(&mut self) -> Result<Vec<Token>, CompilerError> {
        let current_path: PathBuf = match &self.current_file {
            Some(p) => p.to_path_buf(),
            None => return Err(CompilerError::NonExistentFileError),
        };
        let file: File = match File::open(current_path.clone()) {
            Ok(f) => f,
            Err(e) => {
                return Err(CompilerError::FileIOError(
                    self.current_file
                        .clone()
                        .unwrap_or("Could not get file info...".into()),
                    e,
                ))
            }
        };

        let mut char_reader: CharReader = CharReader::new(&file, &current_path)?;

        let tokens: Vec<Token> = Vec::new();
        let mut last_char: char = ' ';
        let mut built_lexeme: String = String::new();

        for c in char_reader {
            while c.is_ascii_whitespace() {
                continue;
            }
        }

        Ok(tokens)
    }

    fn lex_stdin(&self) -> Result<Vec<Token>, CompilerError> {
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

    pub fn lex(&mut self) -> Result<Vec<Token>, CompilerError> {
        match self.current_file {
            Some(_) => self.lex_file(),
            None => self.lex_stdin(),
        }
    }
}
