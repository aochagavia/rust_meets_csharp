//! Intermediate representation of our C# subset to be run by the interpreter

use ast;

pub struct Program {
    pub entry_point: usize,
    pub methods: Vec<Method>
}

#[derive(Clone)]
pub struct Method {
    //pub params: Vec<()>
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
    pub var_id: usize,
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
    VarRead(usize), // Var id
    NewObject(usize), // Class id
}

#[derive(Clone)]
pub struct FieldAccess {
    pub target: Expression,
    pub field_id: usize
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
    pub method_id: usize,
    pub arguments: Vec<Expression>
}
