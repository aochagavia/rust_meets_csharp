use frontend::analysis::labels;

/// Internal representation of a value
///
/// Note: this resembles `ast::Type`
#[derive(Clone)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    Int(i64),
    Object(Object),
    Null
}

/// Internal representation of an object
#[derive(Clone)]
pub struct Object {
    pub class: labels::ClassDecl,
    pub fields: Vec<Value>,
}
