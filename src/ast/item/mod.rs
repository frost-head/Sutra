use crate::{ast::block::Block, lexer::token::Token};

pub enum Item {
    Function(FuncItem),
    Struct(),
}

pub struct FuncItem {
    pub name: String,
    pub params: Option<Vec<Token>>,
    pub body: Block,
}
