use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
    path::PathBuf,
};

use common::{error::CompilerError, token::Token};

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
        let file: Result<File, _> = match &self.current_file {
            Some(f) => File::open(f),
            None => return Err(CompilerError::NonExistentFileError),
        };
        let file: File = match file {
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

        let tokens: Vec<Token> = Vec::new();
        let mut last_char: char = ' ';

        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    return Err(CompilerError::UnreadableLineError(
                        self.current_file
                            .clone()
                            .unwrap_or("Could not get file info...".into()),
                        self.current_line_number,
                        e,
                    ))
                }
            };

            self.current_line_number += 1;

            let mut built_lexeme: String = String::new();

            for c in line.trim().chars() {
                last_char = c;
                if c.is_alphabetic() {
                    built_lexeme.push(c);
                } else if c.is_ascii_digit() {
                }
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
