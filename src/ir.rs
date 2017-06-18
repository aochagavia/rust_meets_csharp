//! Intermediate representation of our C# subset to be run by the interpreter

use ast;
use analysis::{ClassId, FieldId, MethodId, VarId};

pub struct Program {
    pub methods: Vec<Method>
}

#[derive(Clone)]
pub struct Method {
    pub body: Vec<Statement>
}

#[derive(Clone)]
pub enum Statement {
    Assign(Assign),
    Expression(Expression),
    Return(Option<Expression>),
    VarDecl,
}

#[derive(Clone)]
pub struct Assign {
    pub var_id: VarId,
    pub value: Expression
}

#[derive(Clone)]
pub enum Expression {
    FieldAccess(Box<FieldAccess>),
    Literal(Literal),
    /// BinaryOp desugars into intrinsic
    Intrinsic(Box<Intrinsic>),
    /// New desugars into new object + method call
    MethodCall(MethodCall),
    /// Identifier desugars into a VarRead or MethodCall (for static methods)
    VarRead(VarId),
    NewObject(ClassId),
}

#[derive(Clone)]
pub struct FieldAccess {
    pub target: Expression,
    pub field_id: FieldId
}

#[derive(Clone)]
pub enum Literal {
    Int(i64),
    String(String),
    Array(Vec<Expression>),
    Null
}

#[derive(Clone)]
pub enum Intrinsic {
    IntOp(ast::BinaryOperator, Expression, Expression),
    PrintLine(Expression),
}

#[derive(Clone)]
pub struct MethodCall {
    pub method_id: MethodId,
    pub arguments: Vec<Expression>
}
