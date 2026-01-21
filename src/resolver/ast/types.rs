use crate::{errors::span::Span, resolver::symbol::SymbolId};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeRes {
    Named {
        id: SymbolId,
        span: Span,
    },
    Function {
        params: Vec<TypeRes>,
        return_type: Box<TypeRes>,
        span: Span,
    },
}

impl Display for TypeRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TypeRes::Named { id, .. } => write!(f, "{}", id),
            TypeRes::Function {
                params,
                return_type,
                span: _span,
            } => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)?;
                Ok(())
            }
        }
    }
}

impl TypeRes {
    pub fn print_span(&self) {
        match self {
            TypeRes::Named { id, span } => {
                println!("Named Type: {} at {}", id, span);
            }
            TypeRes::Function {
                params: _,
                return_type: _,
                span,
            } => {
                println!("Function Type at {}", span);
            }
        }
    }
}
