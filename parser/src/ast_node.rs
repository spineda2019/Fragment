use std::fmt::Display;

pub trait ASTNode: Display {
    fn print(&self);
}
