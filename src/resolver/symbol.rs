use crate::resolver::{scope::ScopeId, types::TypeId};

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

pub struct SymbolId(pub usize);

