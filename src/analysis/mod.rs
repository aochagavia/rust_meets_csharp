mod query_engine;

pub use self::query_engine::QueryEngine;

pub struct ClassInfo {
    pub superclass_id: Option<usize>,
    pub name: String,
    pub field_names: Vec<String>,

    // FIXME: should we make a distinction between static and dynamic methods?
    pub methods: Vec<usize>,
}

// FIXME: make the inner field private
#[derive(Clone, Copy, Debug)]
pub struct FieldId(pub usize);

// FIXME: make the inner field private
#[derive(Clone, Copy, Debug)]
pub struct MethodId(pub usize);

// FIXME: make the inner field private
#[derive(Clone, Copy, Debug)]
pub struct VarId(pub usize);

impl VarId {
    pub fn this() -> VarId {
        VarId(0)
    }
}

// FIXME: make the inner field private
#[derive(Clone, Copy, Debug)]
pub struct ClassId(pub usize);

/// Represents the type of an expression
#[derive(Clone, Debug)]
pub enum Type {
    Int,
    String,
    Array(Box<Type>),
    Void,
    Class(ClassId)
}
