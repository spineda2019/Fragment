use crate::ast_node::ASTNode;

struct Ast {
    node: Box<dyn ASTNode>,
}
