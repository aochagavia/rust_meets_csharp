//! Intermediate representation of our C# subset to be run by the interpreter

use frontend::analysis::labels;
use frontend::ast::BinaryOperator;

#[derive(Clone, Debug, Copy)]
pub struct FieldId(pub(crate) usize);

#[derive(Clone, Debug, Copy)]
pub struct MethodId(pub(crate) usize);

#[derive(Clone, Debug, Copy)]
pub struct VarId(pub(crate) usize);

impl VarId {
    pub fn this() -> VarId {
        VarId(0)
    }
}

pub struct Program {
    pub methods: Vec<Method>,
    pub entry_point: MethodId
}

#[derive(Clone, Debug)]
pub struct Method {
    pub body: Vec<Statement>
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assign(Assign),
    Expression(Expression),
    Return(Option<Expression>),
    VarDecl,
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub var_id: VarId,
    pub value: Expression
}

#[derive(Clone, Debug)]
pub enum Expression {
    FieldAccess(Box<FieldAccess>),
    Literal(Literal),
    /// BinaryOp desugars into intrinsic
    Intrinsic(Box<Intrinsic>),
    /// New desugars into new object + method call
    MethodCall(MethodCall),
    /// Identifier desugars into a VarRead or MethodCall (for static methods)
    VarRead(VarId),
    NewObject(labels::ClassDecl),
}

#[derive(Clone, Debug)]
pub struct FieldAccess {
    pub target: Expression,
    pub field_id: FieldId
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(i64),
    String(String),
    Array(Vec<Expression>),
    Null
}

#[derive(Clone, Debug)]
pub enum Intrinsic {
    IntOp(BinaryOperator, Expression, Expression),
    PrintLine(Expression),
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub method_id: MethodId,
    pub arguments: Vec<Expression>
}
