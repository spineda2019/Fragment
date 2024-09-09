use std::fmt::Display;

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

impl Display for FunctionPrototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = format!(
            "Function Prototype Name: {}\nFunction Prototype Args: {:?}",
            self.name, self.args
        );
        write!(f, "{}", display)
    }
}

impl ASTNode for FunctionPrototype {
    fn print(&self) {
        println!("Node: Function Prototype");
        println!("Function Prototype: {}", self.name);
        println!("Function args: {:?}", self.args);
    }
}
