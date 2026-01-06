use std::collections::HashMap;

use crate::resolver::{
    scope::{Scope, ScopeId},
    symbol::Symbol,
};

pub mod scope;
pub mod symbol;
pub mod types;

pub struct Resolver {
    pub scopes: Vec<Scope>,
    pub symbols: Vec<Symbol>,
    pub cur_scope: ScopeId,
}

impl Resolver {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        let mut symbols = Vec::new();
        let cur_scope = ScopeId(0);

        scopes.push(Scope {
            symbols: HashMap::new(),
            parent: None,
        });

        Resolver {
            scopes,
            symbols,
            cur_scope,
        }
    }
}
