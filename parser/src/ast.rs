use common::{error::CompilerError, operator_precedence::OperatorPrecedence, token::Token};
use lexer::lexer::Lexer;

use crate::{
    ast_node::ASTNode,
    ast_nodes::{
        expressions::{
            numeric_expression::NumericExpression, variable_expression::VariableExpression,
        },
        functions::{function_definition::Function, function_prototype::FunctionPrototype},
    },
};

pub struct Ast<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Ast<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            current_token: Token::BeginningOfFile,
        }
    }

    fn eat_current_token_and_advance_lexer(&mut self) -> Result<(), CompilerError> {
        self.current_token = self.lexer.get_token()?;
        Ok(())
    }

    fn parse_number_expression(&mut self) -> Result<Box<NumericExpression>, CompilerError> {
        self.current_token = self.lexer.get_token()?;
        match self.current_token {
            Token::F64Literal(number) => Ok(Box::new(NumericExpression::new(number))),
            _ => Err(CompilerError::ExpectedNumberError),
        }
    }

    fn parse_identifier(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        // current token should be an identifier
        let id_string: String = match &self.current_token {
            Token::Identifier(id) => id.to_owned(),
            _ => {
                return Err(CompilerError::UnexpectedTokenError(
                    self.current_token.clone(),
                ));
            }
        };

        self.eat_current_token_and_advance_lexer()?;

        if let Token::LeftParenthesis = &self.current_token {
            self.eat_current_token_and_advance_lexer()?; // eat '('
            let mut expressions: Vec<Box<dyn ASTNode>> = Vec::new();

            if self.current_token == Token::RightParenthesis {
                loop {
                    expressions.push(self.parse_expression()?);

                    if self.current_token == Token::RightParenthesis {
                        break;
                    }

                    if self.current_token != Token::Comma {
                        return Err(CompilerError::UnexpectedTokenError(
                            self.current_token.clone(),
                        ));
                    }
                }
            }
        } else {
            return Ok(Box::new(VariableExpression::new(&id_string)));
        }

        todo!()
    }

    fn parse_primary(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        match &self.current_token {
            Token::Identifier(_) => Ok(self.parse_identifier()?),
            Token::F64Literal(_) => Ok(self.parse_number_expression()?),
            Token::LeftParenthesis => Ok(self.parse_parenthesis_expression()?),
            token => Err(CompilerError::UnexpectedTokenError(token.clone())),
        }
    }

    fn parse_binary_operation_rhs(
        &mut self,
        precedence: OperatorPrecedence,
        lhs: Box<dyn ASTNode>,
    ) -> Result<Box<dyn ASTNode>, CompilerError> {
        todo!()
    }

    fn parse_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        let lhs: Box<dyn ASTNode> = self.parse_primary()?;
        self.parse_binary_operation_rhs(OperatorPrecedence::new(0), lhs)
    }

    fn parse_parenthesis_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        if let Token::LeftParenthesis = self.current_token {
            self.eat_current_token_and_advance_lexer()?;

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

    fn parse_prototype(&mut self) -> Result<Box<FunctionPrototype>, CompilerError> {
        if let Token::Identifier(id) = &self.current_token {
            let function_name: String = id.to_string();

            self.eat_current_token_and_advance_lexer()?;

            if self.current_token != Token::LeftParenthesis {
                return Err(CompilerError::UnexpectedTokenError(
                    self.current_token.clone(),
                ));
            }

            self.eat_current_token_and_advance_lexer()?;

            let mut args: Vec<String> = Vec::new();
            while let Token::Identifier(id) = &self.current_token {
                args.push(id.to_string());
                self.eat_current_token_and_advance_lexer()?;
            }

            if self.current_token != Token::RightParenthesis {
                return Err(CompilerError::UnexpectedTokenError(
                    self.current_token.clone(),
                ));
            }

            self.eat_current_token_and_advance_lexer()?;
            Ok(Box::new(FunctionPrototype::new(&function_name, args)))
        } else {
            Err(CompilerError::FunctionNameNotFound)
        }
    }

    fn parse_definition(&mut self) -> Result<Box<Function>, CompilerError> {
        if self.current_token != Token::Def {
            return Err(CompilerError::UnexpectedTokenError(
                self.current_token.clone(),
            ));
        }
        self.eat_current_token_and_advance_lexer()?;
        let prototype: Box<FunctionPrototype> = self.parse_prototype()?;
        let definition_expression = self.parse_expression()?;

        Ok(Box::new(Function::new(prototype, definition_expression)))
    }

    fn handle_definition(&mut self) -> Result<(), CompilerError> {
        let defintion_node = self.parse_definition()?;
        defintion_node.print();

        Ok(())
    }

    pub fn parse_tokens(&mut self) -> Result<(), CompilerError> {
        println!("In the parsing tokens stage!");
        self.eat_current_token_and_advance_lexer()?; // eat the beginning of file token

        loop {
            println!("\t{:#?}", self.current_token);
            match self.current_token {
                Token::Eof => break,
                Token::SemiColon => {
                    self.eat_current_token_and_advance_lexer()?;
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
