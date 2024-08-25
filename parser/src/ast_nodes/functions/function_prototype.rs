use crate::ast_node::ASTNode;

pub struct FunctionPrototype {
    name: String,
    args: Vec<String>,
}

impl FunctionPrototype {
    pub fn new(name: &str, args: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            args,
        }
    }
}

impl ASTNode for FunctionPrototype {}
