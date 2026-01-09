use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ast::item::function::FuncItem,
    errors::ResolverError,
    resolver::{
        scope::{Scope, ScopeId},
        symbol::{Symbol, SymbolId, SymbolKind},
        types::{Type, TypeId, TypeKind, TypeTable},
    },
};

pub mod scope;
pub mod symbol;
pub mod types;

#[derive(Debug)]
pub struct Resolver {
    pub scopes: Vec<Scope>,
    pub symbols: Vec<Symbol>,
    pub cur_scope: ScopeId,
    pub type_table: TypeTable,
}

impl Resolver {
    pub fn new(type_table: TypeTable) -> Self {
        let mut scopes = Vec::new();
        scopes.push(Scope {
            symbols: HashMap::new(),
            parent: None,
        });
        let symbols = Vec::new();
        let cur_scope = ScopeId(0);

        Resolver {
            scopes,
            symbols,
            cur_scope,
            type_table,
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

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.cur_scope.0].parent {
            self.cur_scope = parent;
        }
    }

    pub fn declare_symbol(&mut self, symbol: Symbol) -> Result<()> {
        let symbol_id = self.symbols.len();
        let scope = &mut self.scopes[self.cur_scope.0];

        self.symbols.push(symbol.clone());
        if scope.symbols.contains_key(&symbol.name) {
            return Err(ResolverError::SymbolAlreadyDeclared {
                symbol: symbol.name.clone(),
            }
            .into());
        }

        scope
            .symbols
            .insert(symbol.name.clone(), SymbolId(symbol_id));
        Ok(())
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

    pub fn declare_function(&mut self, func_item: FuncItem) -> Result<()> {
        let mut return_id = None;
        let mut param_ids: Option<Vec<TypeId>> = None;
        if let Some(return_type) = func_item.return_type.clone() {
            let return_kind = Type::type_ref_to_type(return_type.type_ref)?;
            return_id = Some(self.type_table.intern(return_kind));
        }
        if let Some(param_types) = func_item.params.clone() {
            param_ids = Some(
                param_types
                    .into_iter()
                    .map(|param_type| -> Result<TypeId> {
                        let param_kind = Type::type_ref_to_type(param_type.type_ref)?;
                        Ok(self.type_table.intern(param_kind))
                    })
                    .collect::<Result<Vec<TypeId>>>()?,
            );
        }
        let func_type = TypeKind::Function {
            params: param_ids,
            ret: return_id,
        };
        let func_id = self.type_table.intern(func_type);
        let func_sym = Symbol {
            name: func_item.name.clone(),
            kind: SymbolKind::Function,
            type_id: func_id,
            scope_id: self.cur_scope.clone(),
            mutable: false,
        };
        self.declare_symbol(func_sym)?;
        Ok(())
    }
}
