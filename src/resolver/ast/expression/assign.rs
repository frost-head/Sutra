use core::fmt;

use crate::resolver::symbol::SymbolId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub ident: SymbolId,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}
