use crate::error::CompilerError;

#[derive(Debug)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    F64Literal(i64),
    UnaryOperator(UnaryOperater),
    DivisionOperator,
    Unknown(char),
}

#[derive(Debug)]
pub enum UnaryOperater {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl UnaryOperater {
    pub fn new(operator: char) -> Result<Self, CompilerError> {
        match operator {
            '+' => Ok(UnaryOperater::Addition),
            '-' => Ok(UnaryOperater::Subtraction),
            '*' => Ok(UnaryOperater::Multiplication),
            '/' => Ok(UnaryOperater::Division),
            _ => panic!(),
        }
    }
}
