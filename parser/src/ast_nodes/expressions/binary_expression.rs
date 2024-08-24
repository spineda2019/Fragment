use common::token::UnaryOperater;

use crate::ast_node::ASTNode;

struct BinaryExpression {
    operator: UnaryOperater,
    left_hand_side: Box<dyn ASTNode>,
    right_hand_side: Box<dyn ASTNode>,
}

impl BinaryExpression {
    pub fn new(
        operator: UnaryOperater,
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
