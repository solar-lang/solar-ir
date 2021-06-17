use crate::TypeIdent;

/// ConcreteType is basically a c-struct.
pub struct ConcreteType {
    pub name: TypeIdent,
    pub fields: Vec<(RefType, TypeIdent)>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RefType {
    Direct,
    Ref,
    RefMut,
}
