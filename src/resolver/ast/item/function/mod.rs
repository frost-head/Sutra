use anyhow::{Context, Result};
use core::fmt;

use crate::{
    parser::ast::item::function::FuncItem as PaserFuncItem,
    resolver::{
        ast::{
            block::Block,
            item::function::{fn_params::Param, fn_return::FnReturn},
        },
        symbol::SymbolId,
    },
};

pub mod fn_params;
pub mod fn_return;

#[derive(Debug, Clone)]
pub struct FuncItem {
    pub id: SymbolId,
    pub params: Option<Vec<Param>>,
    pub return_type: Option<FnReturn>,
    pub body: Block,
}

impl FuncItem {
    pub fn new(
        id: SymbolId,
        params: Option<Vec<Param>>,
        return_type: Option<FnReturn>,
        body: Block,
    ) -> Self {
        FuncItem {
            id,
            params,
            return_type,
            body,
        }
    }
}

impl fmt::Display for FuncItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(", self.id)?;
        if let Some(params) = &self.params {
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", param)?;
            }
        }
        if let Some(return_type) = &self.return_type {
            write!(f, ") -> {} \n{}\n", return_type, self.body)?;
        } else {
            write!(f, ") \n{}\n", self.body)?;
        }
        Ok(())
    }
}
