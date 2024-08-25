use std::fmt::Debug;

use crate::error::CompilerError;

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
}

#[derive(Debug)]
pub enum SimpleBinaryOperater {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    GreaterThan,
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
            Token::LeftParenthesis => String::from("Token: ("),
            Token::RightParenthesis => String::from("Token: )"),
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
