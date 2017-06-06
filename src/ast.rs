pub struct Program {
    pub items: Vec<TopItem>
}

pub enum Type {
    Array(Box<Type>),
    Int,
    String,
    Void,
    Custom(String)
}

/// Top-level items
pub enum TopItem {
    /// Class declaration
    ClassDecl { name: String, inherits_from: Option<String>, items: Vec<ClassItem> },
}

/// Class items
pub enum ClassItem {
    /// Field declaration
    FieldDecl { name: String, ty: Type, assignment: Option<Expression> },
    /// Method declaration
    MethodDecl { name: String, params: Vec<(String, Type)>, body: Vec<Statement>, return_ty: Type }
}

/// Statements
pub enum Statement {
    /// Assignment
    Assign { var_name: String, expr: Expression },
    /// Expression
    Expression(Expression),
    /// Return
    Return(Option<Expression>),
    /// Variable declaration
    VarDecl { var_name: String, expr: Option<Expression> },
}

/// Expressions
pub enum Expression {
    /// Binary operators
    BinaryOp { operator: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
    /// Field access
    FieldAccess { variable: String, field_name: String },
    /// Method call
    MethodCall { variable: String, method_name: String, params: Vec<Expression> },
    /// Static method call
    StaticMethodCall { class_name: String, method_name: String, params: Vec<Expression> },
    /// Literals
    Literal(Literal),
    /// Variables
    VarRead(String),
}

/// Literals
pub enum Literal {
    Int(i64),
    /// String
    String(String),
    /// List
    Array(Vec<Expression>)
}

/// Operators
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div
}
