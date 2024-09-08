use common::{error::CompilerError, token::Token};
use lexer::lexer::Lexer;

use crate::{
    ast_node::ASTNode,
    ast_nodes::{
        expressions::numeric_expression::NumericExpression,
        expressions::numeric_expression::VariableExpression,
        functions::function_prototype::FunctionPrototype,
    },
};

pub struct Ast {
    lexer: Lexer,
    current_token: Token,
}

impl Ast {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            current_token: Token::BeginningOfFile,
        }
    }

    fn eat_current_token_and_advance_lexer(&mut self) -> Result<(), CompilerError> {
        self.current_token = self.lexer.get_token()?;
        Ok(())
    }

    fn parse_number_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        self.current_token = self.lexer.get_token()?;
        match self.current_token {
            Token::F64Literal(number) => Ok(Box::new(NumericExpression::new(number))),
            _ => Err(CompilerError::ExpectedNumberError),
        }
    }

    fn parse_identifier(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        // current token should be an identifier
        if let Token::Identifier(id) = self.current_token {
            self.eat_current_token_and_advance_lexer()?;

            if let Token::LeftParenthesis = self.current_token {
                todo!();
                return Ok(Box::new(VariableExpression::new(id)));
            }

            todo!()
        } else {
            return Err(CompilerError::UnexpectedTokenError(
                self.current_token.clone(),
            ));
        }
    }

    fn parse_primary(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        match &self.current_token {
            Token::Identifier(_) => Ok(self.parse_identifier()?),
            Token::F64Literal(_) => Ok(self.parse_number_expression()?),
            Token::LeftParenthesis => Ok(self.parse_parenthesis_expression()?),
            token => Err(CompilerError::UnexpectedTokenError(token.clone())),
        }
    }

    fn parse_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        let lhs = self.parse_primary()?;
        todo!()
    }

    fn parse_parenthesis_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        self.current_token = self.lexer.get_token()?;

        if let Token::LeftParenthesis = self.current_token {
            let v: Box<dyn ASTNode> = self.parse_expression()?;

            // should now be a ')' from parse_expression call
            match self.current_token {
                Token::RightParenthesis => {}
                _ => {
                    return Err(CompilerError::UnexpectedTokenError(
                        self.current_token.clone(),
                    ))
                }
            };

            self.current_token = self.lexer.get_token()?;
            Ok(v)
        } else {
            Err(CompilerError::UnexpectedTokenError(
                self.current_token.clone(),
            ))
        }
    }

    fn handle_extern(&self) {}

    fn handle_top_level_expression(&self) {}

    fn parse_prototype(&self) -> Result<Box<FunctionPrototype>, CompilerError> {
        if let Token::Identifier(_) = self.current_token {
            todo!()
        } else {
            return Err(CompilerError::FunctionNameNotFound);
        }
    }

    fn handle_definition(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        self.current_token = self.lexer.get_token()?;
        todo!()
    }

    pub fn parse_tokens(&mut self) -> Result<(), CompilerError> {
        println!("In the parsing tokens stage!");

        loop {
            println!("\t{:#?}", self.current_token);
            match self.current_token {
                Token::Eof => break,
                Token::SemiColon => {
                    self.current_token = self.lexer.get_token()?;
                }
                Token::Def => {
                    self.handle_definition()?;
                }
                Token::Extern => self.handle_extern(),
                _ => self.handle_top_level_expression(),
            };
        }

        Ok(())
    }
}
