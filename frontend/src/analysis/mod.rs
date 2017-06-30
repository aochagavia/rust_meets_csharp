mod on_demand;
mod preprocess;

use ast::Label;

pub use self::on_demand::query_engine::QueryEngine;
pub use self::preprocess::ast_preprocessor::AstPreprocessor;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Node;

pub struct IntrinsicInfo;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FieldId(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MethodId(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct VarId(usize);

impl VarId {
    pub fn this() -> VarId {
        VarId(0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ClassId(usize);

/// Represents the type of an expression
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    Int,
    String,
    Array(TypeId),
    Void,
    Class(Label)
}
