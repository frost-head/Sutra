use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::resolver::{
    scope::{Scope, ScopeId},
    symbol::{Symbol, SymbolId},
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

        Resolver {
            scopes,
            symbols,
            cur_scope,
        }
    }

    pub fn enter_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId(self.scopes.len());
        self.scopes.push(Scope {
            symbols: HashMap::new(),
            parent,
        });
        self.cur_scope = id.clone();
        id
    }

    pub fn exit_scope(&mut self) -> Result<()> {
        let parent = self.scopes[self.cur_scope.0].parent;
        self.cur_scope = parent.context("Failed to exit scope")?;
        Ok(())
    }

    pub fn declare_symbol(&mut self, symbol: Symbol) {
        let symbol_id = self.symbols.len();
        self.symbols.push(symbol.clone());

        self.scopes[self.cur_scope.0]
            .symbols
            .insert(symbol.name, SymbolId(symbol_id));
    }
    
    pub fn resolve_symbol(&self, name: &str) -> Option<SymbolId> {
        let mut current_scope = self.cur_scope;

        while let Some(scope) = self.scopes.get(current_scope.0) {
            if let Some(symbol_id) = scope.symbols.get(name) {
                return Some(symbol_id.clone());
            }
            current_scope = scope.parent?;
        }

        None
    }
}
