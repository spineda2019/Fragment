pub struct OperatorPrecedence {
    precedence: usize,
}

impl OperatorPrecedence {
    pub fn new(precedence: usize) -> OperatorPrecedence {
        OperatorPrecedence { precedence }
    }
}
