use crate::ast_node::ASTNode;

struct FunctionCallExpression {
    calle: String,
    args: Vec<Box<dyn ASTNode>>,
}

impl FunctionCallExpression {
    pub fn new(calle: &str, args: Vec<Box<dyn ASTNode>>) -> Self {
        Self {
            calle: calle.to_string(),
            args,
        }
    }
}

impl ASTNode for FunctionCallExpression {}
