use crate::ast_node::ASTNode;

struct NumericExpression {
    value: f64,
}

impl NumericExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl ASTNode for NumericExpression {}
