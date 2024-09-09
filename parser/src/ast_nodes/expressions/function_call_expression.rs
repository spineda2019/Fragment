use std::fmt::Display;

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

impl Display for FunctionCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args_string: String = String::with_capacity(self.args.len());
        for arg in &self.args {
            args_string = format!("{} {}", args_string, arg);
        }

        write!(
            f,
            "FunctionCallExpression calle: {}\nFunctionCallExpression args: {}",
            self.calle, args_string
        )
    }
}

impl ASTNode for FunctionCallExpression {
    fn print(&self) {
        let mut args_string: String = String::with_capacity(self.args.len());
        for arg in &self.args {
            args_string = format!("{} {}", args_string, arg);
        }

        println!("Node: FunctionCallExpression");
        println!("FunctionCallExpression calle: {}", self.calle);
        println!("FunctionCallExpression args: {}", args_string);
    }
}
