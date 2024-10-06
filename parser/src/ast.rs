use common::{
    error::CompilerError,
    operator_precedence::OperatorPrecedence,
    token::{SimpleBinaryOperater, Token},
};
use lexer::lexer::Lexer;

use crate::{
    ast_node::ASTNode,
    ast_nodes::{
        expressions::{
            binary_expression::BinaryExpression, function_call_expression::FunctionCallExpression,
            numeric_expression::NumericExpression, variable_expression::VariableExpression,
        },
        functions::{function_definition::Function, function_prototype::FunctionPrototype},
    },
};

pub struct Ast<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    verbose: bool,
}

impl<'a> Ast<'a> {
    pub fn new(lexer: &'a mut Lexer, verbose: bool) -> Self {
        Self {
            lexer,
            current_token: Token::BeginningOfFile,
            verbose,
        }
    }

    fn eat_current_token_and_advance_lexer(&mut self) -> Result<(), CompilerError> {
        if self.verbose {
            println!("\n*** About to Eat Token ***");
            println!("{:?}", self.current_token);
            println!("*** Token Eaten ***\n");
        }
        self.current_token = self.lexer.get_token()?;
        Ok(())
    }

    fn parse_number_expression(&mut self) -> Result<Box<NumericExpression>, CompilerError> {
        match self.current_token {
            Token::F64Literal(number) => {
                self.eat_current_token_and_advance_lexer()?;
                Ok(Box::new(NumericExpression::new(number)))
            }
            _ => Err(CompilerError::ExpectedNumberError(
                self.lexer.current_line(),
                self.lexer.current_file(),
            )),
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

        // eat identifier
        self.eat_current_token_and_advance_lexer()?;
        let mut expressions: Vec<Box<dyn ASTNode>> = Vec::new();

        if let Token::LeftParenthesis = &self.current_token {
            self.eat_current_token_and_advance_lexer()?; // eat '('

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

        self.eat_current_token_and_advance_lexer()?;
        Ok(Box::new(FunctionCallExpression::new(
            &id_string,
            expressions,
        )))
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
        mut lhs: Box<dyn ASTNode>,
    ) -> Result<Box<dyn ASTNode>, CompilerError> {
        loop {
            if self.verbose {
                println!("Parsing binary expression. LHS:\n{}", lhs);
            }
            let current_token_precedence: OperatorPrecedence =
                OperatorPrecedence::new(&self.current_token);

            if current_token_precedence.get_precedence() < precedence.get_precedence() {
                if self.verbose {
                    println!("While parsing binop, this token:");
                    println!("{:?}", self.current_token);
                    println!("Had a lower precedence than:");
                    println!("{}\n", precedence.get_precedence());
                }
                return Ok(lhs);
            } else {
                let binary_operator: SimpleBinaryOperater =
                    SimpleBinaryOperater::from_token(&self.current_token)?;

                // eat operator
                self.eat_current_token_and_advance_lexer()?;

                let mut rhs = self.parse_primary()?;

                let next_precedence = OperatorPrecedence::new(&self.current_token);

                if current_token_precedence.get_precedence() < next_precedence.get_precedence() {
                    rhs = self.parse_binary_operation_rhs(
                        OperatorPrecedence::increment_other(&current_token_precedence),
                        rhs,
                    )?;
                }

                lhs = Box::new(BinaryExpression::new(binary_operator, lhs, rhs));
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Box<dyn ASTNode>, CompilerError> {
        let lhs: Box<dyn ASTNode> = self.parse_primary()?;
        self.parse_binary_operation_rhs(OperatorPrecedence::from_number(0), lhs)
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

    fn parse_extern(&mut self) -> Result<Box<FunctionPrototype>, CompilerError> {
        // eat extern token
        self.eat_current_token_and_advance_lexer()?;
        self.parse_prototype()
    }

    fn handle_extern(&mut self) -> Result<(), CompilerError> {
        let parse_node = self.parse_extern()?;
        parse_node.print();
        if self.verbose {
            println!(
                "Succesfully parsed definition! Current Token: {:?}",
                self.current_token
            );
        }
        Ok(())
    }

    fn parse_top_level_expression(&mut self) -> Result<Box<Function>, CompilerError> {
        let expression = self.parse_expression()?;
        let prototype: Box<FunctionPrototype> = Box::new(FunctionPrototype::new("", Vec::new()));
        Ok(Box::new(Function::new(prototype, expression)))
    }

    fn handle_top_level_expression(&mut self) -> Result<(), CompilerError> {
        let parse_node = self.parse_top_level_expression()?;
        parse_node.print();
        if self.verbose {
            println!(
                "Succesfully parsed top level expression! Current Token: {:?}",
                self.current_token
            );
        }
        Ok(())
    }

    fn parse_prototype(&mut self) -> Result<Box<FunctionPrototype>, CompilerError> {
        if self.verbose {
            println!("Start parsing prototype!");
        }
        if let Token::Identifier(id) = &self.current_token {
            let function_name: String = id.to_string();

            // eat prototype name
            self.eat_current_token_and_advance_lexer()?;

            if self.current_token != Token::LeftParenthesis {
                return Err(CompilerError::UnexpectedTokenError(
                    self.current_token.clone(),
                ));
            }

            self.eat_current_token_and_advance_lexer()?;

            let mut args: Vec<String> = Vec::new();
            while let Token::Identifier(arg) = &self.current_token {
                if self.verbose {
                    println!("parse_protoype: Prototype arg found: {}\n", arg);
                }
                args.push(arg.to_string());
                self.eat_current_token_and_advance_lexer()?;
            }

            if self.current_token != Token::RightParenthesis {
                return Err(CompilerError::UnexpectedTokenError(
                    self.current_token.clone(),
                ));
            }

            // eat ) token
            self.eat_current_token_and_advance_lexer()?;
            if self.verbose {
                println!("prototype parsed!!");
            }
            Ok(Box::new(FunctionPrototype::new(&function_name, args)))
        } else {
            Err(CompilerError::FunctionNameNotFound)
        }
    }

    fn parse_definition(&mut self) -> Result<Box<Function>, CompilerError> {
        if self.verbose {
            println!("Start parsing definition!");
        }
        if self.current_token != Token::Def {
            return Err(CompilerError::UnexpectedTokenError(
                self.current_token.clone(),
            ));
        }

        // eat Def token and expect func name
        self.eat_current_token_and_advance_lexer()?;

        let prototype: Box<FunctionPrototype> = self.parse_prototype()?;
        let definition_expression = self.parse_expression()?;

        if self.verbose {
            println!("Finished parsing definition!");
        }
        Ok(Box::new(Function::new(prototype, definition_expression)))
    }

    fn handle_definition(&mut self) -> Result<(), CompilerError> {
        if self.verbose {
            println!("Start handling def!");
        }
        let defintion_node = self.parse_definition()?;
        defintion_node.print();

        if self.verbose {
            println!(
                "Succesfully parsed definition! Current Token: {:?}",
                self.current_token
            );
        }

        Ok(())
    }

    /// Parse tokens in the source file
    ///
    /// # Arguments
    /// * `self` - a mutable reference to self - a Parser that owns a lexer buffer and a current
    /// token
    pub fn parse_tokens(&mut self) -> Result<(), CompilerError> {
        if self.verbose {
            println!("***************************************************************************");
            println!("*                       In the parsing tokens stage!                      *");
            println!("***************************************************************************");
            println!(" ");
        }
        self.eat_current_token_and_advance_lexer()?; // eat the beginning of file token

        loop {
            if self.verbose {
                println!("*********************************************************************\n");
            }
            match self.current_token {
                Token::Eof => break,
                Token::SemiColon => {
                    self.eat_current_token_and_advance_lexer()?;
                }
                Token::Def => {
                    self.handle_definition()?;
                }
                Token::Extern => self.handle_extern()?,
                _ => self.handle_top_level_expression()?,
            };
        }

        Ok(())
    }
}
