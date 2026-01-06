use crate::resolver::{scope::ScopeId, types::TypeId};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_id: TypeId,
    pub scope_id: ScopeId,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Function,
    Variable,
    Type,
}

#[derive(Debug, Clone)]
pub struct SymbolId(pub usize);
