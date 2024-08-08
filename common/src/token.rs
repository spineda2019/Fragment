pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    F64Literal(i64),
}
