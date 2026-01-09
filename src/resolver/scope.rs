use crate::resolver::symbol::SymbolId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);

#[derive(Debug)]
pub struct Scope {
    pub symbols: HashMap<String, SymbolId>,
    pub parent: Option<ScopeId>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
        }
    }
}
