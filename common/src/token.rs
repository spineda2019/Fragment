use std::fmt::Debug;

use crate::error::CompilerError;

#[derive(PartialEq)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    F64Literal(f64),
    SimpleBinaryOperator(SimpleBinaryOperater),
    Unknown(char),
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    BeginningOfFile,
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleBinaryOperater {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    GreaterThan,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Eof => Token::Eof,
            Token::Def => Token::Def,
            Token::Extern => Token::Extern,
            Token::Identifier(s) => Token::Identifier(s.clone()),
            Token::F64Literal(f) => Token::F64Literal(*f),
            Token::SimpleBinaryOperator(op) => Token::SimpleBinaryOperator(op.clone()),
            Token::Unknown(c) => Token::Unknown(*c),
            Token::LeftParenthesis => Token::LeftParenthesis,
            Token::RightParenthesis => Token::RightParenthesis,
            Token::SemiColon => Token::SemiColon,
            Token::Comma => Token::Comma,
            Token::BeginningOfFile => Token::BeginningOfFile,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = match self {
            Token::Eof => String::from("Token: End of file token"),
            Token::Def => String::from("Token: keyword -> def"),
            Token::Extern => String::from("Token: keyword -> extern"),
            Token::Identifier(i) => format!("Token: identifier -> {}", i),
            Token::F64Literal(f) => format!("Token: f64 literal -> {}", f),
            Token::SimpleBinaryOperator(op) => {
                format!("Token: Simple binary operator -> {}", op.to_char())
            }
            Token::Unknown(u) => format!("Unknown token: {}", u),
            Token::LeftParenthesis => String::from("Token: Left Parenthesis -> ("),
            Token::RightParenthesis => String::from("Token: Right Parenthesis -> )"),
            Token::SemiColon => String::from("Token: Semicolon -> ;"),
            Token::BeginningOfFile => String::from("Beginning of file"),
            Token::Comma => String::from("Token: Comma -> ,"),
        };
        write!(f, "{}", message)
    }
}

impl SimpleBinaryOperater {
    pub fn new(operator: char) -> Result<Self, CompilerError> {
        match operator {
            '+' => Ok(SimpleBinaryOperater::Addition),
            '-' => Ok(SimpleBinaryOperater::Subtraction),
            '*' => Ok(SimpleBinaryOperater::Multiplication),
            '/' => Ok(SimpleBinaryOperater::Division),
            '<' => Ok(SimpleBinaryOperater::LessThan),
            '>' => Ok(SimpleBinaryOperater::GreaterThan),
            c => Err(CompilerError::InvalidOperaterCharacter(c)),
        }
    }

    pub fn from_token(operator: &Token) -> Result<Self, CompilerError> {
        match operator {
            Token::SimpleBinaryOperator(o) => Ok(o.clone()),
            _ => Err(CompilerError::InvalidOperaterCharacter(' ')),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Addition => '+',
            Self::Subtraction => '-',
            Self::Multiplication => '*',
            Self::Division => '/',
            Self::LessThan => '<',
            Self::GreaterThan => '>',
        }
    }
}
