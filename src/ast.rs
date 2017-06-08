use std::cell::Cell;
use std::fmt;

use pretty::PrettyPrinter;

pub type Label = u32;

#[derive(Debug)]
pub enum Type {
    Array(Box<Type>),
    Int,
    String,
    Void,
    Custom(String)
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Type::Array(ref ty) => write!(f, "{}[]", ty),
            &Type::Int => write!(f, "int"),
            &Type::String => write!(f, "string"),
            &Type::Void => write!(f, "void"),
            &Type::Custom(ref s) => write!(f, "{}", s)
        }
    }
}

pub fn fresh_label() -> Label {
    thread_local! {
        static NEXT_LABEL: Cell<u32> = Cell::new(0);
    }

    let label = NEXT_LABEL.with(|l| {
        let current = l.get();
        l.set(current + 1);
        current
    });

    label
}

// A program
#[derive(Debug)]
pub struct Program {
    pub items: Vec<TopItem>
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrinter::new().print_program(f, self)
    }
}

/// Top-level items
#[derive(Debug)]
pub enum TopItem {
    /// Class declaration
    ClassDecl(ClassDecl),
}

#[derive(Debug)]
pub struct ClassDecl {
    pub label: Label,
    pub name: String,
    pub superclass: Option<String>,
    pub items: Vec<ClassItem>
}

/// Class items
#[derive(Debug)]
pub enum ClassItem {
    /// Field declaration
    FieldDecl(FieldDecl),
    /// Method declaration
    MethodDecl(MethodDecl)
}

#[derive(Debug)]
pub struct FieldDecl {
    pub label: Label,
    pub name: String,
    pub ty: Type,
    pub assignment: Option<Expression>
}

#[derive(Debug)]
pub struct MethodDecl {
    pub label: Label,
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub body: Vec<Statement>,
    pub return_ty: Type
}

/// Statements
#[derive(Debug)]
pub enum Statement {
    /// Assignment
    Assign(Assign),
    /// Expression
    Expression(Expression),
    /// Return
    Return(Return),
    /// Variable declaration
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub struct Assign {
    pub label: Label,
    pub var_name: String,
    pub expr: Expression
}

#[derive(Debug)]
pub struct Return {
    pub label: Label,
    pub expr: Option<Expression>
}

#[derive(Debug)]
pub struct VarDecl {
    pub var_name: String,
    pub ty: Type,
    pub expr: Option<Expression>
}

/// Expressions
#[derive(Debug)]
pub enum Expression {
    /// Binary operators
    BinaryOp(BinaryOp),
    /// Field access
    FieldAccess(FieldAccess),
    /// Literals
    Literal(Literal),
    /// Method call (may be static)
    MethodCall(MethodCall),
    /// New (construct class and allocate it on the heap)
    New(New),
    /// Variables
    VarRead(String),
}

#[derive(Debug)]
pub struct BinaryOp {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct FieldAccess {
    pub var_name: String,
    pub field_name: String,
}

#[derive(Debug)]
pub struct MethodCall {
    /// Target can be a variable name, field name or class name
    pub target: String,
    pub method_name: String,
    pub params: Vec<Expression>
}

#[derive(Debug)]
pub struct New {
    pub class_name: String,
    pub params: Vec<Expression>
}

/// Literals
#[derive(Debug)]
pub enum Literal {
    Int(i64),
    /// String
    String(String),
    /// List
    Array(Vec<Expression>)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Literal::Int(x) => x.fmt(f),
            Literal::String(ref s) => write!(f, "\"{}\"", s),
            Literal::Array(_) => unimplemented!()
        }
    }
}

/// Operators
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOperator::Add => "+".fmt(f),
            BinaryOperator::Sub => "-".fmt(f),
            BinaryOperator::Mul => "*".fmt(f),
            BinaryOperator::Div => "/".fmt(f)
        }
    }
}
