use crate::resolver::symbol::SymbolId;
use std::collections::HashMap;

pub struct ScopeId(pub usize);

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
