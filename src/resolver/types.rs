pub struct TypeId(pub usize);
pub enum TypeKind {
    Int,
    Bool,
    Float,
    Pointer(TypeId),
    Function {
        params: Vec<TypeId>,
        ret: TypeId,
    },
    Struct {
        name: String,
        fields: Vec<(String, TypeId)>,
    },
    Alias(TypeId),
}

pub struct Type {
    pub kind: TypeKind,
}

pub struct TypeTable {
    pub types: Vec<Type>,
}

impl TypeTable {
    pub fn new() -> Self {
        TypeTable { types: Vec::new() }
    }

    pub fn intern(&mut self, kind: TypeKind) -> TypeId {
        let id = TypeId(self.types.len());
        self.types.push(Type { kind });
        id
    }
}
