pub mod labels;
mod on_demand;
mod preprocess;

pub use self::on_demand::query_engine::QueryEngine;
pub use self::preprocess::ast_preprocessor::AstPreprocessor;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

/// Represents the type of an expression
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    Int,
    String,
    Array(TypeId),
    Void,
    Class(self::labels::ClassDecl)
}
