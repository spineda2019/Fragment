use crate::ast_node::ASTNode;

struct BinaryExpression {
    left_hand_side: Box<dyn ASTNode>,
    right_hand_side: Box<dyn ASTNode>,
}
