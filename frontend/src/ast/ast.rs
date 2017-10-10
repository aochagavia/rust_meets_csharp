#![allow(dead_code)]

use std::cell::Cell;
use std::fmt;

use analysis::labels;
use super::pretty::PrettyPrinter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Label(u32);

#[derive(Clone, Debug)]
pub enum Type {
    Array(Box<Type>),
    Custom(String),
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Type::Array(ref ty) => write!(f, "{}[]", ty),
            &Type::Custom(ref s) => write!(f, "{}", s),
            &Type::Void => write!(f, "void"),
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

    Label(label)
}

// A program
#[derive(Clone, Debug)]
pub struct Program {
    pub items: Vec<TopItem>
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrinter::new().print_program(f, self)
    }
}

impl Program {
    pub fn classes<'a>(&'a self) -> impl Iterator<Item=&'a ClassDecl> {
        self.items.iter().map(|&TopItem::ClassDecl(ref cd)| cd)
    }

    pub fn methods<'a>(&'a self) -> impl Iterator<Item=&'a MethodDecl> {
        self.classes()
            .flat_map(|cd| cd.items.iter()) // Get a stream of ClassItem
            .filter_map(|ci| match *ci { ClassItem::MethodDecl(ref md) => Some(md), _ => None }) // Get a stream of MethodDecl
    }
}

/// Top-level items
#[derive(Clone, Debug)]
pub enum TopItem {
    /// Class declaration
    ClassDecl(ClassDecl),
}

impl fmt::Display for TopItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrinter::new().print_top_item(f, self)
    }
}

#[derive(Clone, Debug)]
pub struct ClassDecl {
    pub label: Label,
    pub name: String,
    pub items: Vec<ClassItem>
}

impl ClassDecl {
    pub fn find_field(&self, name: &str) -> Label {
        self.items.iter()
                  .filter_map(|i| i.field_decl())
                  .find(|fd| &fd.name == name)
                  .unwrap().label
    }

    pub fn find_method_any<'a>(&'a self, name: &str) -> &'a MethodDecl {
        self.items.iter().filter_map(|i| i.method_decl()).find(|md| &md.name == name).unwrap()
    }

    pub fn find_method(&self, is_static: bool, name: &str) -> Label {
        self.items.iter()
                  .filter_map(|i| i.method_decl())
                  .find(|md| &md.name == name && md.is_static == is_static)
                  .unwrap().label
    }
}

/// Class items
#[derive(Clone, Debug)]
pub enum ClassItem {
    /// Field declaration
    FieldDecl(FieldDecl),
    /// Method declaration
    MethodDecl(MethodDecl)
}

impl ClassItem {
    pub fn method_decl(&self) -> Option<&MethodDecl> {
        match *self {
            ClassItem::MethodDecl(ref m) => Some(m),
            _ => None
        }
    }

    pub fn field_decl(&self) -> Option<&FieldDecl> {
        match *self {
            ClassItem::FieldDecl(ref f) => Some(f),
            _ => None
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldDecl {
    pub label: Label,
    pub name: String,
    pub ty: Type,
    pub assignment: Option<Expression>
}

#[derive(Clone, Debug)]
pub struct MethodDecl {
    pub label: Label,
    pub name: String,
    pub params: Vec<VarDecl>,
    pub body: Vec<Statement>,
    pub is_static: bool,
    pub return_ty: Type
}

/// StatementsFAssign
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Assign {
    pub label: Label,
    pub var_name: String,
    pub expr: Expression
}

#[derive(Clone, Debug)]
pub struct Return {
    pub label: Label,
    pub expr: Option<Expression>
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub label: Label,
    pub var_name: String,
    pub ty: Type,
    pub expr: Option<Expression>
}

/// Expressions
#[derive(Clone, Debug)]
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
    /// Identifiers
    ///
    /// Represents a variable usage or a class name when calling a static method
    Identifier(Identifier),
    /// The `this` keyword
    This(This),
}

#[derive(Clone, Debug)]
pub struct BinaryOp {
    pub label: Label,
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct FieldAccess {
    pub label: Label,
    pub target: Box<Expression>,
    pub field_name: String,
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub label: Label,
    pub kind: LiteralKind
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub label: Label,
    pub target: Box<Expression>,
    pub method_name: String,
    pub args: Vec<Expression>
}

#[derive(Clone, Debug)]
pub struct New {
    pub label: Label,
    pub class_name: String
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub label: Label,
    pub name: String
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct This {
    pub label: Label
}

impl Expression {
    pub fn label(&self) -> labels::Expression {
        match *self {
            Expression::BinaryOp(BinaryOp { label, .. })
            | Expression::FieldAccess(FieldAccess { label, .. })
            | Expression::Literal(Literal { label, .. })
            | Expression::MethodCall(MethodCall { label, .. })
            | Expression::New(New { label, .. })
            | Expression::Identifier(Identifier { label, .. })
            | Expression::This(This { label, .. })
            => labels::Expression(label)
        }
    }

    pub fn identifier(&self) -> &Identifier {
        match self {
            &Expression::Identifier(ref i) => i,
            _ => panic!()
        }
    }
}

/// Literals
#[derive(Clone, Debug)]
pub enum LiteralKind {
    /// Int
    Int(i64),
    /// String
    String(String),
    /// List
    Array(Type, Vec<Expression>),
    /// Null
    Null
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            LiteralKind::Int(x) => x.fmt(f),
            LiteralKind::String(ref s) => write!(f, "\"{}\"", s),
            LiteralKind::Null => "null".fmt(f),
            LiteralKind::Array(_, _) => unimplemented!()
        }
    }
}

/// Operators
#[derive(Clone, Copy, Debug)]
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
