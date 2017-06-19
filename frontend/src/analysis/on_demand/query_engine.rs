use std::collections::HashMap;
use std::collections::hash_map::Entry;

use analysis::{AstPreprocessor, ClassInfo, IntrinsicInfo, ClassId, FieldId, MethodId, TypeId, VarId};
use ast::*;
use super::type_map::TypeMap;

pub struct QueryEngine<'a> {
    pub preprocessor: AstPreprocessor,
    program: &'a mut Program,
    types: TypeMap
}

#[allow(unused_variables)]
impl<'a> QueryEngine<'a> {
    pub fn new(program: &'a mut Program) -> QueryEngine<'a> {
        // FIXME: populate the tables with type information of intrinsic methods
        let preprocessor = AstPreprocessor::new(program);
        QueryEngine { program, types: TypeMap::default(), preprocessor }
    }

    pub fn types(&self) -> &TypeMap {
        &self.types
    }

    pub fn types_mut(&mut self) -> &mut TypeMap {
        &mut self.types
    }

    pub fn intrinsics() -> Vec<IntrinsicInfo> {
        // Note: this is a temporary hack. We may use it while there is only one intrinsic,\
        // but if more intrinsics are added this will need to be changed

        // Potential additional intrinsics that may be useful:
        // * Console.Write
        // * Console.ReadLine
        vec![IntrinsicInfo]
    }

    pub fn query_class_info(&mut self, class_id: ClassId) -> Vec<ClassInfo> {
        // Search in our class map. Should always be present
        unimplemented!()
    }

    pub fn query_field(&mut self, class_id: ClassId, field_name: &str) -> FieldId {
        unimplemented!()
    }

    pub fn query_method(&mut self, class_id: ClassId, method_name: &str) -> Option<MethodId> {
        unimplemented!()
    }

    pub fn query_param_types(&mut self, method_id: MethodId) -> Vec<TypeId> {
        unimplemented!()
    }

    pub fn query_constructor(&mut self, class_id: ClassId) -> MethodId {
        unimplemented!()
    }

    pub fn query_class(&mut self, class_name: &str) -> Option<ClassId> {
        // Search in our class map. If not present, search through all class declarations.
        unimplemented!()
    }

    pub fn query_var_decl(&mut self, var_use: Label) -> VarId {
        // The label may correspong to a VarAssign, VarDecl or VarRead
        unimplemented!()
    }

    pub fn query_var(&mut self, identifier: Label) -> VarId {
        // The label may correspong to a VarAssign, VarDecl or VarRead
        unimplemented!()
    }

    pub fn query_var_type(&mut self, identifier: VarId) -> TypeId {
        // The label corresponds to an identifier
        unimplemented!()
    }

    pub fn query_parent_node(&mut self, node: Label) -> Option<Label> {
        unimplemented!()
    }

    pub fn query_parent_method(&mut self, node: Label) -> MethodId {
        // Note: should work for statements and expressions. Panics otherwise.
        unimplemented!()
    }

    pub fn query_return_type(&mut self, method: MethodId) -> TypeId {
        unimplemented!()
    }

    pub fn query_is_static(&mut self, method: MethodId) -> bool {
        unimplemented!()
    }

    /// Returns the type of an expression, or `None` in case the label
    /// corresponds to another kind of node. Note: references to undefined
    /// variables have no type.
    pub fn query_expr_type(&mut self, expr: Label) -> Option<TypeId> {
        unimplemented!()
    }
}
