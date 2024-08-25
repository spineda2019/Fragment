use std::{
    fmt::{format, Debug, Display},
    path::PathBuf,
};

pub enum CompilerError {
    FileIOError(PathBuf, std::io::Error),
    UnrecognizedFileError(PathBuf),
    UnreadableLineError(PathBuf, usize, std::io::Error),
    UnreadableCharacterError(usize),
    NonExistentFileError,
    InvalidNumberError(String),
    ExpectedNumberError,
    ExpectedExpressionError,
    InvalidOperaterCharacter(char),
}

impl CompilerError {
    fn error_message(&self) -> String {
        match self {
            CompilerError::FileIOError(f, e) => {
                format!(
                    "Error ocurred while opening file: {:?}\nError was: {}",
                    f, e
                )
            }
            CompilerError::UnrecognizedFileError(f) => {
                format!("{:?} is not a valid Fragment source file", f)
            }
            CompilerError::UnreadableLineError(file, line_number, e) => {
                format!(
                    "Error ocurred while reading line.\nFile: {:?}\nLine: {}\nError: {}",
                    file, line_number, e
                )
            }
            CompilerError::NonExistentFileError => String::from(
                "The compiler tried reading a file while not holding an instance of one",
            ),
            CompilerError::UnreadableCharacterError(char_number) => {
                format!(
                    "Encountered an unreadable character at file position: {}",
                    char_number
                )
            }
            CompilerError::InvalidNumberError(number) => {
                format!("Invalid numeric format: {}", number)
            }
            CompilerError::ExpectedNumberError => {
                String::from("Expected to find number expression, but none were found")
            }
            CompilerError::ExpectedExpressionError => {
                String::from("An expression was expected, but not found...")
            }
            CompilerError::InvalidOperaterCharacter(c) => {
                format!("{} is not a valid operater character", c)
            }
        }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = self.error_message();
        write!(f, "{}", message)
    }
}

impl Debug for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.error_message();
        write!(f, "{}", message)
    }
}
