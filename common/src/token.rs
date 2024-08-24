#[derive(Debug)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    F64Literal(i64),
    PlusOperator,
    MinusOperator,
    MultiplicationOperator,
    DivisionOperator,
    Unknown(char),
}
