use crate::token::SimpleBinaryOperater;
use crate::token::Token;

pub struct OperatorPrecedence {
    precedence: isize,
}

impl OperatorPrecedence {
    pub fn new(precedence: &Token) -> OperatorPrecedence {
        OperatorPrecedence {
            precedence: match precedence {
                Token::SimpleBinaryOperator(SimpleBinaryOperater::LessThan) => 10,
                Token::SimpleBinaryOperator(SimpleBinaryOperater::Addition) => 20,
                Token::SimpleBinaryOperator(SimpleBinaryOperater::Subtraction) => 20,
                Token::SimpleBinaryOperator(SimpleBinaryOperater::Multiplication) => 40,
                _ => -1,
            },
        }
    }

    pub fn from_number(number: isize) -> OperatorPrecedence {
        OperatorPrecedence { precedence: number }
    }

    pub fn get_precedence(&self) -> isize {
        self.precedence
    }

    pub fn increment_other(other: &OperatorPrecedence) -> OperatorPrecedence {
        OperatorPrecedence {
            precedence: other.get_precedence() + 1,
        }
    }
}
