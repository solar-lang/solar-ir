mod scope;
mod generate_ir;
use std::collections::HashMap;
mod function;
use function::ConcreteFunction;
mod ident;
use ident::{TypeIdent, AbsIdent };
mod ty;
use ty::ConcreteType;

pub struct ConcreteScope {
    pub functions: HashMap<AbsIdent, ConcreteFunction>,
    pub types: HashMap<TypeIdent, ConcreteType>
}

