use crate::parser::ast::item::function::FuncItem;

pub mod function;

#[derive(Debug, Clone)]
pub enum Item {
    Function(FuncItem),
    Struct(),
}
