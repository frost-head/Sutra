use crate::ast::item::function::FuncItem;

pub mod function;

pub enum Item {
    Function(FuncItem),
    Struct(),
}
