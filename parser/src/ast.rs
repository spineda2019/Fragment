use std::collections::VecDeque;

use common::{error::CompilerError, token::Token};

use crate::{ast_node::ASTNode, ast_nodes::expressions::numeric_expression::NumericExpression};

pub struct Ast {
    tokens: VecDeque<Token>,
}

impl Ast {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse_number_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        match self.tokens.pop_front() {
            Some(Token::F64Literal(number)) => Ok(Box::new(NumericExpression::new(number))),
            _ => Err(CompilerError::ExpectedNumberError),
        }
    }

    pub fn parse_parenthesis_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        let current_token: Token = match self.tokens.pop_front() {
            Some(t) => t,
            None => return Err(CompilerError::ExpectedExpressionError),
        };

        todo!()
    }
}
