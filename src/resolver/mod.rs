use std::collections::HashMap;

use anyhow::Result;

use crate::errors::span::Span;
use crate::resolver::ast::Ast as ResolvedAst;
use crate::resolver::ast::item::Item as ResolverItem;
use crate::resolver::ast::item::function::FuncItem as ResolverFunc;
use crate::resolver::ast::item::function::fn_params::Param;
use crate::resolver::ast::item::function::fn_return::FnReturn;
use crate::{
    errors::ResolverError,
    resolver::{
        scope::{Scope, ScopeId},
        symbol::{Symbol, SymbolId, SymbolKind},
    },
    utils::indent_multiline,
};
use crate::{
    parser::ast::{
        Ast,
        block::Block,
        item::{Item, function::FuncItem},
        statement::Stmt,
        types::TypeRef,
    },
    resolver::ast::types::TypeRes,
};

pub mod ast;
pub mod scope;
pub mod symbol;

#[derive(Debug)]
pub struct Resolver {
    pub scopes: Vec<Scope>,
    pub symbols: Vec<Symbol>,
    pub cur_scope: ScopeId,
    pub ast: ResolvedAst,
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
            children: Vec::new(),
        });
        let mut symbols = Vec::new();
        let cur_scope = ScopeId(0);

        scopes[0]
            .symbols
            .insert("int".to_string(), SymbolId(symbols.len()));
        symbols.push(Symbol {
            name: "int".to_string(),
            kind: SymbolKind::PremitiveType(symbol::PremitiveTypes::Int),
            scope_id: cur_scope,
            mutable: false,
        });

        scopes[0]
            .symbols
            .insert("float".to_string(), SymbolId(symbols.len()));
        symbols.push(Symbol {
            name: "float".to_string(),
            kind: SymbolKind::PremitiveType(symbol::PremitiveTypes::Float),
            scope_id: cur_scope,
            mutable: false,
        });

        scopes[0]
            .symbols
            .insert("bool".to_string(), SymbolId(symbols.len()));

        symbols.push(Symbol {
            name: "bool".to_string(),
            kind: SymbolKind::PremitiveType(symbol::PremitiveTypes::Bool),
            scope_id: cur_scope,
            mutable: false,
        });
        let ast = ResolvedAst::new();

        Resolver {
            scopes,
            symbols,
            cur_scope,
            ast,
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

        for item in &ast.items {
            match item {
                Item::Function(func_item) => {
                    self.funct_tree(func_item.clone())?;
                }

                _ => {
                    eprintln!("Error occurred while parsing the input");
                    std::process::exit(1);
                }
            }
        }

        Ok(())
    }

    pub fn funct_tree(&mut self, func_item: FuncItem) -> Result<()> {
        let id = self
            .resolve_symbol(&func_item.clone().name, None)
            .expect(format!("Failed to resolve symbol {}", &func_item.name).as_str());

        let mut params = Vec::new();
        if let Some(param) = func_item.params {
            for p in param {
                let type_res = self.type_ref_to_type_res(p.type_ref);
                let symbol_id = self
                    .resolve_symbol(&p.name, None)
                    .expect(format!("Failed to resolve symbol {}", &p.name).as_str());
                params.push(Param {
                    id: symbol_id,
                    type_res,
                    span: p.span,
                })
            }
        }

        let return_type = func_item.return_type;

        let fn_return: Option<FnReturn> = match return_type {
            Some(return_type) => Some(FnReturn {
                type_res: self.type_ref_to_type_res(return_type.type_ref),
                span: return_type.span,
            }),
            None => None,
        };

        let params = if params.len() == 0 {
            None
        } else {
            Some(params)
        };

        // let statements = Vec::new();

        for stmt in func_item.body.statements {
            match stmt {
                Stmt::LetStmt(let_statement) => {
                    println!("cur_scope : {:?}", self.cur_scope);
                    println!("Statement: {:?}\n", let_statement);
                    let func_sym = self
                        .resolve_symbol(&func_item.name, None)
                        .expect(format!("Failed to resolve function {}", &func_item.name).as_str());

                    let sym = self
                        .symbols
                        .get(func_sym.0)
                        .expect(format!("Could not get the symbol {}", &func_item.name).as_str());

                    println!(
                        "self.scopes[sym.scope_id.0]: {:?}",
                        self.scopes[sym.scope_id.0]
                    );

                    let sid = self.resolve_symbol(&let_statement.identifier, None);
                    println!("Symbol ID: {:?}", sid);
                }
                Stmt::ReturnStmt(return_statement) => todo!(),
                Stmt::Expr(expression) => todo!(),
            }
        }

        // let func_item = ResolverFunc::new(id, params, return_type: fn_return,Block();
        // );
        // self.ast.items.push(ResolverItem::Function(func_item));

        Ok(())
    }

    pub fn type_ref_to_type_res(&mut self, type_ref: TypeRef) -> TypeRes {
        match type_ref {
            TypeRef::Named { name, span } => TypeRes::Named {
                id: self
                    .resolve_symbol(&name, None)
                    .expect(format!("Failed to resolve type {}", &name).as_str()),
                span,
            },
        }
    }

    pub fn enter_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId(self.scopes.len());
        self.scopes.push(Scope {
            symbols: HashMap::new(),
            parent,
            children: Vec::new(),
        });
        self.scopes[self.cur_scope.0].children.push(id);
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

    pub fn resolve_symbol(&self, name: &str, scope_id: Option<ScopeId>) -> Option<SymbolId> {
        let mut current_scope = self.cur_scope;
        if let Some(scope_id) = scope_id {
            current_scope = scope_id;
        }

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
