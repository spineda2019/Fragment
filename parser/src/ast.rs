use common::{error::CompilerError, token::Token};
use lexer::lexer::Lexer;

use crate::{ast_node::ASTNode, ast_nodes::expressions::numeric_expression::NumericExpression};

pub struct Ast {
    lexer: Lexer,
}

impl Ast {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse_number_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        match self.lexer.get_token()? {
            Token::F64Literal(number) => Ok(Box::new(NumericExpression::new(number))),
            _ => Err(CompilerError::ExpectedNumberError),
        }
    }

    pub fn parse_parenthesis_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        let current_token: Token = self.lexer.get_token()?;

        todo!()
    }

    pub fn parse_tokens(&mut self) -> Result<(), CompilerError> {
        println!("In the parsing tokens stage!");
        let mut token: Token = self.lexer.get_token()?;

        dbg!(&token);

        loop {
            match token {
                Token::Eof => break,
                other => {
                    println!("\t{:#?}", other);
                    token = self.lexer.get_token()?;
                }
            };
        }

        Ok(())
    }
}
