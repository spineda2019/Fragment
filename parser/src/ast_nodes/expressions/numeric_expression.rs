use std::fmt::Display;

use crate::ast_node::ASTNode;

pub struct NumericExpression {
    value: f64,
}

impl NumericExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Display for NumericExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumericExpression: {}", self.value)
    }
}

impl ASTNode for NumericExpression {
    fn print(&self) {
        println!("Node: NumericExpression");
        println!("NumericExpression value: {}", self.value);
    }
}
