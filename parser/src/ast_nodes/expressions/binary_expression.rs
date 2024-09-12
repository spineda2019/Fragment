use std::fmt::Display;

use common::token::SimpleBinaryOperater;

use crate::ast_node::ASTNode;

pub struct BinaryExpression {
    operator: SimpleBinaryOperater,
    left_hand_side: Box<dyn ASTNode>,
    right_hand_side: Box<dyn ASTNode>,
}

impl BinaryExpression {
    pub fn new(
        operator: SimpleBinaryOperater,
        left_hand_side: Box<dyn ASTNode>,
        right_hand_side: Box<dyn ASTNode>,
    ) -> Self {
        Self {
            operator,
            left_hand_side,
            right_hand_side,
        }
    }
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Operator: {}\nLHS: {}\nRHS: {}",
            self.operator.to_char(),
            self.left_hand_side,
            self.right_hand_side
        )
    }
}

impl ASTNode for BinaryExpression {
    fn print(&self) {
        println!("Node: Binary Expression");
        println!("Binary Expression Operator: {}", self.operator.to_char());
        println!("Binary Expression LHS: {}", self.left_hand_side);
        println!("Binary Expression RHS: {}", self.right_hand_side);
    }
}
