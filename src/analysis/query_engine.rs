use ast::Label;
use super::{ClassInfo, ClassId, FieldId, MethodId, VarId};

pub struct QueryEngine {

}

impl QueryEngine {
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

    pub fn query_var(&mut self, identifier: Label) -> VarId {
        unimplemented!()
    }

    /// Returns the type of an expression, or `None` in case the label
    /// corresponds to another kind of node. Note that static name variables
    /// are not expressions. Therefore, a label pointing to a static variable
    /// name will return `None`.
    pub fn query_expr_type(&mut self, expr: Label) -> Option<&super::Type> {

        None
    }
}
