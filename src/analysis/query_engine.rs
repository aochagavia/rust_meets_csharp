use ast::*;
use super::{ClassInfo, IntrinsicInfo, ClassId, FieldId, MethodId, VarId};

pub struct QueryEngine<'a> {
    program: &'a Program
}

#[allow(unused_variables)]
impl<'a> QueryEngine<'a> {
    pub fn new(program: &'a Program) -> QueryEngine<'a> {
        // FIXME: populate the tables with type information so intrinsics
        // can be properly analyzed
        QueryEngine { program }
    }

    pub fn intrinsics() -> Vec<IntrinsicInfo> {
        unimplemented!()
    }

    pub fn methods(&self) -> Vec<&'a MethodDecl> {
        unimplemented!()
    }

    pub fn query_entry_point(&mut self) -> MethodId {
        unimplemented!()
    }

    pub fn query_class_info(&mut self, class_id: ClassId) -> Vec<ClassInfo> {
        unimplemented!()
    }

    pub fn query_field(&mut self, class_id: ClassId, field_name: &str) -> FieldId {
        unimplemented!()
    }

    pub fn query_method(&mut self, class_id: ClassId, method_name: &str) -> MethodId {
        unimplemented!()
    }

    pub fn query_constructor(&mut self, class_id: ClassId) -> MethodId {
        unimplemented!()
    }

    pub fn query_class(&mut self, class_name: &str) -> Option<ClassId> {
        unimplemented!()
    }

    pub fn query_var_decl(&mut self, identifier: Label) -> VarId {
        unimplemented!()
    }

    pub fn query_var(&mut self, identifier: Label) -> VarId {
        unimplemented!()
    }

    pub fn query_var_type(&mut self, identifier: VarId) -> super::Type {
        unimplemented!()
    }

    pub fn query_parent_method(&mut self, node: Label) -> MethodId {
        // Note: should work for statements and expressions. Panics otherwise.
        unimplemented!()
    }

    pub fn query_return_type(&mut self, method: MethodId) -> super::Type {
        unimplemented!()
    }

    pub fn query_is_static(&mut self, method: MethodId) -> bool {
        unimplemented!()
    }


    /// Returns the type of an expression, or `None` in case the label
    /// corresponds to another kind of node. Note that static name variables
    /// are not expressions. Therefore, a label pointing to a static variable
    /// name will return `None`.
    pub fn query_expr_type(&mut self, expr: Label) -> Option<super::Type> {

        None
    }
}
