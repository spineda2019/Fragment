use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use common::{
    error::CompilerError,
    token::{SimpleBinaryOperater, Token},
};

use crate::char_reader::CharReader;

pub struct Lexer {
    current_char_reader: Option<CharReader>,
    current_line_number: usize,
}

impl Lexer {
    const SIMPLE_BINARY_OPERATORS: [char; 6] = ['+', '-', '*', '/', '<', '>'];

    pub fn init() -> Lexer {
        Lexer {
            current_char_reader: None,
            current_line_number: 0,
        }
    }

    pub fn new(file: PathBuf) -> Result<Lexer, CompilerError> {
        Ok(Lexer {
            current_char_reader: Some(CharReader::new(file)?),
            current_line_number: 0,
        })
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

        self.current_char_reader = Some(CharReader::new(file)?);
        self.current_line_number = 0;
        Ok(())
    }

    fn collect_number(&mut self, built_lexeme: &mut String) -> Result<Token, CompilerError> {
        let reader: &mut CharReader = match &mut self.current_char_reader {
            Some(r) => r,
            None => return Err(CompilerError::NonExistentFileError),
        };

        while let Some(c) = reader.preview_char() {
            if c.is_ascii_digit() || c == '.' {
                built_lexeme.push(reader.getchar().unwrap());
            } else {
                break;
            }
        }

        match built_lexeme.parse::<f64>() {
            Ok(n) => Ok(Token::F64Literal(n)),
            Err(_) => Err(CompilerError::ExpectedNumberError),
        }
    }

    fn collect_identifier(&mut self, built_lexeme: &mut String) -> Result<Token, CompilerError> {
        let reader: &mut CharReader = match &mut self.current_char_reader {
            Some(r) => r,
            None => return Err(CompilerError::NonExistentFileError),
        };

        while let Some(c) = reader.preview_char() {
            if c.is_ascii_alphabetic() {
                built_lexeme.push(reader.getchar().unwrap());
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

    pub fn get_token(&mut self) -> Result<Token, CompilerError> {
        let last_char: char;

        let reader: &mut CharReader = match &mut self.current_char_reader {
            Some(r) => r,
            None => return Err(CompilerError::NonExistentFileError),
        };

        loop {
            match reader.getchar() {
                Some(c) if c.is_ascii_whitespace() => {
                    if c == '\n' {
                        self.current_line_number += 1;
                    }
                    continue;
                }
                Some('(') => return Ok(Token::LeftParenthesis),
                Some(')') => return Ok(Token::RightParenthesis),
                Some(';') => return Ok(Token::SemiColon),
                Some(',') => return Ok(Token::Comma),
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
                self.collect_identifier(&mut built_lexeme)
            }
            digit if digit.is_ascii_digit() || digit == '.' => {
                built_lexeme.push(digit);
                self.collect_number(&mut built_lexeme)
            }
            '#' => {
                while let Some(c) = reader.getchar() {
                    if c == '\n' || c == '\r' {
                        self.current_line_number += 1;
                        return self.get_token();
                    }
                }

                Ok(Token::Eof)
            }
            c if Self::SIMPLE_BINARY_OPERATORS.contains(&c) => {
                Ok(Token::SimpleBinaryOperator(SimpleBinaryOperater::new(c)?))
            }
            unknown => Ok(Token::Unknown(unknown)),
        }
    }

    fn lex_file(&mut self) -> Result<VecDeque<Token>, CompilerError> {
        let mut tokens: VecDeque<Token> = VecDeque::new();
        loop {
            let token: Token = self.get_token()?;
            match token {
                Token::Eof => {
                    tokens.push_back(token);
                    break;
                }
                _ => {
                    tokens.push_back(token);
                    continue;
                }
            };
        }

        Ok(tokens)
    }

    fn lex_stdin(&self) -> Result<VecDeque<Token>, CompilerError> {
        let mut line: String = String::new();
        let tokens: Vec<Token> = Vec::new();
        loop {
            print!("Fragment REPL >> ");
            line.clear();
            let _ = stdout().flush();
            if stdin().read_line(&mut line).is_err() {
                continue;
            } else {
                for token in &tokens {
                    println!("{:?}", token);
                }
            }
        }
    }

    pub fn lex(&mut self) -> Result<VecDeque<Token>, CompilerError> {
        match self.current_char_reader {
            Some(_) => self.lex_file(),
            None => self.lex_stdin(),
        }
    }
}
