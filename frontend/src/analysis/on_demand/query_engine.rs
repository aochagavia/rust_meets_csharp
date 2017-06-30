use std::collections::HashMap;

use analysis::{labels, AstPreprocessor, IntrinsicInfo, TypeId};
use ast::*;
use super::type_map::TypeMap;

pub struct QueryEngine<'a> {
    pub preprocessor: AstPreprocessor,
    pub nodes: HashMap<Label, Node>,
    program: &'a Program,
    types: TypeMap
}

#[allow(unused_variables)]
impl<'a> QueryEngine<'a> {
    pub fn new(program: &'a Program) -> QueryEngine<'a> {
        // FIXME: populate the tables with type information of intrinsic methods
        let preprocessor = AstPreprocessor::new(program);
        QueryEngine {
            program,
            types: TypeMap::default(),
            preprocessor,
            nodes: HashMap::new()
        }
    }

    pub fn types(&self) -> &TypeMap {
        &self.types
    }

    pub fn types_mut(&mut self) -> &mut TypeMap {
        &mut self.types
    }

    pub fn intrinsics() -> Vec<IntrinsicInfo> {
        // Additional intrinsics that may be useful:
        // * Console.Write
        // * Console.ReadLine
        vec![IntrinsicInfo { label: labels::MethodDecl(fresh_label()) }]
    }

    pub fn query_entry_point(&mut self) -> labels::MethodDecl {
        self.preprocessor.entry_point()
    }

    pub fn query_field(&mut self, var_use: labels::VarUse) -> labels::VarDecl {
        unimplemented!()
    }

    pub fn query_method_decl(&mut self, method_use: labels::MethodUse) -> labels::MethodDecl {
        // We need name resolution information here... Where do we get it from?
        // We need a HashMap<MethodUse, MethodDecl>
        unimplemented!()
    }

    pub fn query_param_types(&mut self, method: labels::MethodDecl) -> Vec<TypeId> {
        let md: &MethodDecl = self.nodes[&method.as_label()].downcast();
        unimplemented!()
    }

    pub fn query_constructor(&mut self, class: labels::ClassDecl) -> labels::MethodDecl {
        let cd: &ClassDecl = self.nodes[&class.as_label()].downcast();
        unimplemented!()
    }

    pub fn query_class_decl(&mut self, class_use: labels::TypeUse) -> labels::ClassDecl {
        // Search in our class map. If not present, search through all class declarations
        unimplemented!()
    }

    pub fn query_var_decl(&mut self, var_use: Label) -> labels::VarDecl {
        // The label may correspong to a VarAssign, VarDecl or VarRead
        unimplemented!()
    }

    pub fn query_var_type(&mut self, identifier: labels::VarDecl) -> TypeId {
        unimplemented!()
    }

    pub fn query_parent_node(&mut self, node: Label) -> Option<Label> {
        unimplemented!()
    }

    pub fn query_parent_method(&mut self, node: Label) -> labels::MethodDecl {
        // Note: should work for statements and expressions. Panics otherwise.
        unimplemented!()
    }

    pub fn query_return_type(&mut self, method: labels::MethodDecl) -> TypeId {
        unimplemented!()
    }

    pub fn query_is_static(&mut self, method: labels::MethodDecl) -> bool {
        unimplemented!()
    }

    /// Returns the type of an expression.
    ///
    /// Note: references to undefined variables have no type.
    pub fn query_expr_type(&mut self, expr: labels::Expression) -> Option<TypeId> {
        unimplemented!()
    }
}
