use std::fmt::Display;

use crate::ast_node::ASTNode;

use super::function_prototype::FunctionPrototype;

pub struct Function {
    prototype: Box<FunctionPrototype>,
    body: Box<dyn ASTNode>,
}

impl Function {
    pub fn new(prototype: Box<FunctionPrototype>, body: Box<dyn ASTNode>) -> Self {
        Self { prototype, body }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Function Prototype: {}\nFunction Body: {}",
            self.prototype, self.body
        )
    }
}

impl ASTNode for Function {
    fn print(&self) {
        println!("Node: Function");
        println!("Prototype: {}", self.prototype);
        println!("body: {}", self.body);
    }
}
