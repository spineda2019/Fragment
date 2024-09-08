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

impl ASTNode for VariableExpression {}
