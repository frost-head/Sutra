use std::collections::HashMap;

pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_id: TypeId,
    pub scope_id: ScopeId,
    pub mutable: bool,
}

pub enum SymbolKind {
    Function,
    Variable,
    Type,
}

pub struct ScopeId(usize);
pub struct TypeId(usize);
pub struct SymbolId(usize);

pub struct Scope {
    symbols: HashMap<String, SymbolId>,
    parent: Option<ScopeId>,
}

pub struct Resolver {
    scopes: Vec<Scope>,
    symbols: Vec<Symbol>,
    cur_scope: ScopeId,
}
