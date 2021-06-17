use crate::ident::{AbsIdent, TypeIdent};
use crate::ty::RefType;

pub struct ConcreteFunction {
    pub name: AbsIdent,
    pub arguments: Vec<(AbsIdent, (RefType, TypeIdent))>,
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
pub struct Expression {
    pub steps: Vec<SingleExpression>,
    pub ty: TypeIdent,
}

pub enum SingleExpression {
    ToRef {
        var: AbsIdent,
        ty: (RefType, TypeIdent),
    },
    FunctionCall {
        function_name: AbsIdent,
        arguments: Vec<Expression>,
        ty: (RefType, TypeIdent),
    },
    /// Reinterpret the bits of the expression `expr` as if they were of type `ty`
    Reinterpret {
        expr: Expression,
        ty: (RefType, AbsIdent),
    },
    // Representing constant values might be easiest to do like this.
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
    UnsignedByteArray {length: usize },
    Statement(Statement),
}
