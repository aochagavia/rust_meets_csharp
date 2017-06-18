mod query_engine;

pub use self::query_engine::QueryEngine;

pub struct ClassInfo {
    pub superclass_id: Option<usize>,
    pub name: String,
    pub field_names: Vec<String>,
}

pub struct IntrinsicInfo;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeId(usize);

// FIXME: make the inner field private
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassId(pub usize);

/// Represents the type of an expression
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Int,
    String,
    Array(TypeId),
    Void,
    Class(ClassId)
}
