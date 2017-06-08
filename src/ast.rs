use std::cell::Cell;
use std::fmt;

use pretty::PrettyPrinter;

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Node<TopItem>>
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrinter::new().print_program(f, self)
    }
}

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

#[derive(Debug)]
pub struct Node<T> {
    pub label: u32,
    pub content: T
}

impl<T> From<T> for Node<T> {
    fn from(content: T) -> Node<T> {
        thread_local! {
            static NEXT_LABEL: Cell<u32> = Cell::new(0);
        }

        let label = NEXT_LABEL.with(|l| {
            let current = l.get();
            l.set(current + 1);
            current
        });

        Node { label, content }
    }
}

/// Top-level items
#[derive(Debug)]
pub enum TopItem {
    /// Class declaration
    ClassDecl { name: String, inherits_from: Option<String>, items: Vec<Node<ClassItem>> },
}

/// Class items
#[derive(Debug)]
pub enum ClassItem {
    /// Field declaration
    FieldDecl { name: String, ty: Type, assignment: Option<Expression> },
    /// Method declaration
    MethodDecl { name: String, params: Vec<(String, Type)>, body: Vec<Node<Statement>>, return_ty: Type }
}

/// Statements
#[derive(Debug)]
pub enum Statement {
    /// Assignment
    Assign { var_name: String, expr: Expression },
    /// Expression
    Expression(Expression),
    /// Return
    Return(Option<Expression>),
    /// Variable declaration
    VarDecl { var_name: String, ty: Type, expr: Option<Expression> },
}

/// Expressions
#[derive(Debug)]
pub enum Expression {
    /// Binary operators
    BinaryOp { operator: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
    /// Field access
    FieldAccess { variable: String, field_name: String },
    /// Literals
    Literal(Literal),
    /// Method call (may be static)
    MethodCall { target: String, method_name: String, params: Vec<Expression> },
    /// New (construct class and allocate it on the heap)
    New { class_name: String, params: Vec<Expression> },
    /// Variables
    VarRead(String),
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
            Literal::String(ref s) => s.fmt(f),
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
