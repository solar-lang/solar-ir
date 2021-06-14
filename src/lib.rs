use std::collections::HashMap;

/// Absolute Identifier
#[derive(Eq, PartialEq, Hash)]
pub struct AbsIdent {
    pub path: Vec<String>,
    pub generics: Vec<AbsIdent>,
}

type TypeIdent = AbsIdent;

impl std::fmt::Debug for AbsIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("."))?;

        if self.generics.len() > 0 {
            write!(f, "<")?;
            for (i, ident) in self.generics.iter().enumerate() {
                write!(f, "{:?}", ident)?;

                if i+1 == self.generics.len() {
                    break;
                }

                write!(f, ",")?;
            }
            write!(f, ">")?;
        }

        Ok(())
    }
}

pub struct ConcreteScope {
    pub functions: HashMap<AbsIdent, ConcreteFunction>
}

pub struct ConcreteFunction {
    pub name: AbsIdent,
    pub arguments: Vec<(AbsIdent, TypeIdent)>,
    /// return type
    pub ty: TypeIdent,
}

pub enum Statement {
    /// Introduces a new variable into the scope
    VariableDeclaration {
        variable_name: AbsIdent,
        ty: TypeIdent,
        value: Expression,
    },
    /// TODO: at the moment TODOS are not present in the AST, but are expected to follow
    /// it's a decision, that Assignments should not be counted as expressions. Other options are
    /// just as valid
    Assignment {
        /// Desitination of assignment
        left: Expression,
        /// Value of assignment
        right: Expression,
    },
    /// Recursion lies here. Expressions may hold Statements as well.
    Expression(Expression),

}

// block of expressions yielding a single type as it's final expression
// without polluting the scope in the meantime.
pub struct Expression {}

pub enum SingleExpression {
    FunctionCall {
        function_name: AbsIdent,
        arguments: Vec<Expression>,
        ty: TypeIdent,
    },
    /// Reinterpret the bits of the expression `expr` as if they were of type `ty`
    Reinterpret {
        expr: Expression,
        ty: AbsIdent,
    },
    ConstI64(i64),
    ConstI32(i32),
    ConstI16(i16),
    ConstI8(i8),
    ConstU64(u64),
    ConstU32(u32),
    ConstU16(u16),
    ConstU8(u8),
    ConstF64(f64),
    ConstF32(f32),
    ByteArray {length: usize },
    /// TODO I dont know yet how to represent constant struct expressions yet
    Const {
        ty: TypeIdent,
    },
}
