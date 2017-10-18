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
    Console,
    Array(TypeId),
    Void,
    Class(self::labels::ClassDecl)
}

impl Type {
    pub fn class_decl(&self) -> self::labels::ClassDecl {
        match self {
            &Type::Class(cd) => cd,
            _ => panic!("Type was not a Class type")
        }
    }
}
