use crate::ast_node::ASTNode;

use super::function_prototype::FunctionPrototype;

struct Function {
    prototype: Box<FunctionPrototype>,
    body: Box<dyn ASTNode>,
}

impl Function {
    pub fn new(prototype: Box<FunctionPrototype>, body: Box<dyn ASTNode>) -> Self {
        Self { prototype, body }
    }
}

impl ASTNode for Function {}
