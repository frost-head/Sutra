use core::fmt;
use std::fmt::Formatter;

use crate::{parser::ast::types::TypeRef, errors::TypeRefError, utils::indent_multiline};
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind.clone() {
            TypeKind::Int => write!(f, "int"),
            TypeKind::Bool => write!(f, "bool"),
            TypeKind::Pointer(ty) => write!(f, "*{:?}", ty),
            TypeKind::Function { params, ret } => {
                write!(f, "Function ")?;
                if let Some(params) = params {
                    write!(f, "(")?;
                    for (i, param) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}", param)?;
                    }
                    write!(f, ") ->")?;
                } else {
                    write!(f, "() ->")?;
                }
                if let Some(ret) = ret {
                    write!(f, " {:?}", ret)?;
                    Ok(())
                } else {
                    write!(f, " None")?;
                    Ok(())
                }
            }
            TypeKind::Struct { name, fields } => {
                write!(f, "{} {{", name)?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}: {:?}", field.0, field.1)?;
                }
                write!(f, "}}")?;
                Ok(())
            }
            TypeKind::Alias(ty) => write!(f, "{:?}", ty),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeTable {
    pub types: Vec<Type>,
}

impl fmt::Display for TypeTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, ty) in self.types.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "\n{} : {}, Id : {}",
                indent_multiline(&"Type".to_string(), "    "),
                ty,
                i
            )?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Type {
    pub fn type_ref_to_type(ty: TypeRef) -> Result<TypeKind> {
        match ty {
            TypeRef::Named {
                ref name,
                span: _span,
            } => match name.clone().as_str() {
                "int" => Ok(TypeKind::Int),
                "bool" => Ok(TypeKind::Bool),
                _ => Err(TypeRefError::InvalidTypeReference {
                    type_ref: ty.clone(),
                }
                .into()),
            },

            _ => Err(TypeRefError::InvalidTypeReference { type_ref: ty }.into()),
        }
    }
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
}
