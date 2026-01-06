use crate::ast::item::Item;
pub mod block;
pub mod expression;
pub mod item;
pub mod let_statement;
pub mod return_statement;
pub mod statement;
pub mod types;

pub struct Ast {
    pub items: Vec<Item>,
}
