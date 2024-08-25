use common::token::SimpleBinaryOperater;

use crate::ast_node::ASTNode;

struct BinaryExpression {
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

impl ASTNode for BinaryExpression {}
