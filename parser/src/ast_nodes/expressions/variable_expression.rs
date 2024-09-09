use std::fmt::Display;

use crate::ast_node::ASTNode;

pub struct VariableExpression {
    name: String,
}

impl VariableExpression {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Display for VariableExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable Expression name: {}", self.name)
    }
}

impl ASTNode for VariableExpression {
    fn print(&self) {
        println!("Node: VariableExpression");
        println!("VariableExpression name: {}", self.name);
    }
}
