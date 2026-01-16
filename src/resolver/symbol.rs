use crate::resolver::scope::ScopeId;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope_id: ScopeId,
    pub mutable: bool,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Symbol(name: {},\n kind: {:?},\n scope_id: {:?},\n mutable: {})\n",
            self.name, self.kind, self.scope_id, self.mutable
        )
    }
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Function,
    Variable,
    Type,
}

#[derive(Debug, Clone)]
pub struct SymbolId(pub usize);
