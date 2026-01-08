use crate::{ast::types::TypeRef, errors::TypeRefError};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct TypeId(pub usize);

#[derive(Debug, Clone)]
pub enum TypeKind {
    Int,
    Bool,
    Pointer(TypeId),
    Function {
        params: Option<Vec<TypeId>>,
        ret: Option<TypeId>,
    },
    Struct {
        name: String,
        fields: Vec<(String, TypeId)>,
    },
    Alias(TypeId),
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
}

#[derive(Debug, Clone)]
pub struct TypeTable {
    pub types: Vec<Type>,
}

impl TypeTable {
    pub fn new() -> Self {
        TypeTable { types: Vec::new() }
    }

    // TODO: Convert to Hashmap
    pub fn intern(&mut self, kind: TypeKind) -> TypeId {
        let id = TypeId(self.types.len());
        self.types.push(Type { kind });
        id
    }

    pub fn type_ref_to_type(&mut self, ty: TypeRef) -> Result<TypeId> {
        match ty {
            TypeRef::Named {
                name: _name,
                span: _span,
            } => {
                let type_id = self.intern(TypeKind::Int);
                Ok(type_id)
            }

            _ => Err(TypeRefError::InvalidTypeReference { type_ref: ty }.into()),
        }
    }
}
