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
    current_character_position: usize,
}

impl Lexer {
    pub fn init() -> Lexer {
        Lexer {
            current_file: None,
            current_line_number: 0,
            current_character_position: 0,
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

    fn collect_number(
        &mut self,
        reader: &mut CharReader,
        built_lexeme: &mut String,
    ) -> Result<Token, CompilerError> {
        while let Some(c) = reader.getchar() {
            if c.is_ascii_digit() || c == '.' {
                built_lexeme.push(c);
            } else {
                break;
            }
        }

        match built_lexeme.parse::<i64>() {
            Ok(n) => Ok(Token::F64Literal(n)),
            Err(_) => todo!(),
        }
    }

    fn collect_identifier(
        &mut self,
        reader: &mut CharReader,
        built_lexeme: &mut String,
    ) -> Result<Token, CompilerError> {
        while let Some(c) = reader.getchar() {
            if c.is_ascii_alphabetic() {
                built_lexeme.push(c);
                continue;
            } else {
                break;
            }
        }

        return match built_lexeme.as_str() {
            "def" => Ok(Token::Def),
            "extern" => Ok(Token::Extern),
            _ => Ok(Token::Identifier(built_lexeme.to_string())),
        };
    }

    fn get_token(&mut self, reader: &mut CharReader) -> Result<Token, CompilerError> {
        let mut last_char: char = ' ';

        loop {
            match reader.getchar() {
                Some(c) if c.is_ascii_whitespace() => {
                    if c == '\n' {
                        self.current_line_number += 1;
                    }
                    last_char = c;
                    continue;
                }
                Some(notspace) => {
                    last_char = notspace;
                    break;
                }
                None => {
                    return Ok(Token::Eof);
                }
            };
        }

        let mut built_lexeme = String::new();

        match last_char {
            character if character.is_ascii_alphabetic() => {
                built_lexeme.push(character);
                self.collect_identifier(reader, &mut built_lexeme)
            }
            digit if digit.is_ascii_digit() || digit == '.' => {
                built_lexeme.push(digit);
                self.collect_number(reader, &mut built_lexeme)
            }
            '#' => {
                while let Some(c) = reader.getchar() {
                    if c == '\n' || c == '\r' {
                        self.current_line_number += 1;
                        return self.get_token(reader);
                    }
                }

                Ok(Token::Eof)
            }
            '+' => Ok(Token::PlusOperator),
            '-' => Ok(Token::MinusOperator),
            '*' => Ok(Token::MultiplicationOperator),
            '/' => Ok(Token::DivisionOperator),
            unknown => Ok(Token::Unknown(unknown)),
        }
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

        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token: Token = self.get_token(&mut char_reader)?;
            match token {
                Token::Eof => {
                    tokens.push(token);
                    break;
                }
                _ => {
                    tokens.push(token);
                    continue;
                }
            };
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
