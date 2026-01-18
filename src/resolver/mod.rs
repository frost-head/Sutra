use std::collections::HashMap;

use anyhow::Result;

use crate::{
    errors::ResolverError,
    parser::ast::{
        Ast,
        block::Block,
        item::{Item, function::FuncItem}, statement::Stmt,
    },
    resolver::{
        scope::{Scope, ScopeId},
        symbol::{Symbol, SymbolId, SymbolKind},
    },
    utils::indent_multiline,
};

pub mod scope;
pub mod symbol;
pub mod types;

#[derive(Debug)]
pub struct Resolver {
    pub scopes: Vec<Scope>,
    pub symbols: Vec<Symbol>,
    pub cur_scope: ScopeId,
}
impl std::fmt::Display for Resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolver {{\n\n")?;
        write!(f, "  scopes: {:?},\n\n", self.scopes)?;
        write!(f, "  symbols:\n")?;
        for sym in self.symbols.iter() {
            write!(f, "{}\n\n", indent_multiline(&sym.to_string(), "    "))?;
        }
        write!(f, "  cur_scope: {:?},\n\n", self.cur_scope)
    }
}

impl Resolver {
    pub fn new() -> Self {
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
        }
    }

    pub fn resolve_global(&mut self, ast: Ast) -> Result<(), anyhow::Error> {
        for item in &ast.items {
            match item {
                Item::Function(func_item) => {
                    self.declare_function(func_item.clone())?;
                }

                _ => {
                    eprintln!("Error occurred while parsing the input");
                    std::process::exit(1);
                }
            }
        }
        for item in &ast.items {
            match item {
                Item::Function(func_item) => {
                    self.resolve_function(func_item.clone())?;
                }

                _ => {
                    eprintln!("Error occurred while parsing the input");
                    std::process::exit(1);
                }
            }
        }

        Ok(())
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
        let func_sym = Symbol {
            name: func_item.name.clone(),
            kind: SymbolKind::Function,
            scope_id: self.cur_scope.clone(),
            mutable: false,
        };
        self.declare_symbol(func_sym)?;
        Ok(())
    }

    fn resolve_function(&mut self, fn_item: FuncItem) -> Result<()> {
        let scope_id = self.enter_scope(Some(self.cur_scope));

        self.declare_params(fn_item.clone(), scope_id)?;

        self.resolve_block(fn_item.body)?;

        self.exit_scope();
        Ok(())
    }

    fn resolve_block(&mut self, block: Block) -> Result<()> {
        for item in block.statements {
            match item {
                Stmt::LetStmt(let_statement) => {
                    let sym = Symbol {
                        name: let_statement.identifier,
                        kind: SymbolKind::Variable,
                        scope_id: self.cur_scope,
                        mutable: false,
                    };

                    self.declare_symbol(sym)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn declare_params(
        &mut self,
        fn_item: FuncItem,
        scope_id: ScopeId,
    ) -> Result<(), anyhow::Error> {
        Ok(if let Some(param_types) = fn_item.params.clone() {
            let _ = Some(
                param_types
                    .into_iter()
                    .map(|param_type| -> Result<()> {
                        self.declare_symbol(Symbol {
                            name: param_type.name,
                            kind: SymbolKind::Variable,
                            scope_id,
                            mutable: false,
                        })
                    })
                    .collect::<Result<Vec<()>>>()?,
            );
        })
    }
}
