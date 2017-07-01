use std::collections::HashMap;

use analysis::{labels, AstPreprocessor, IntrinsicInfo, TypeId};
use ast::*;
use super::type_map::TypeMap;

pub struct QueryEngine<'a> {
    pub preprocessor: AstPreprocessor,
    pub nodes: HashMap<Label, Node>,
    program: &'a Program,
    types: TypeMap,
    classes: HashMap<TypeId, labels::ClassDecl>,
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
            nodes: HashMap::new(),
            classes: HashMap::new()
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
        // Get the ClassDecl of the target
        let target_label = self.nodes[&var_use.as_label()].downcast::<FieldAccess>().target.label();
        // Note: a field target should always be an expression and have a type
        let target_ty = self.query_expr_type(target_label).unwrap();
        let target_decl_label = self.classes[&target_ty];
        let target_decl: &ClassDecl = self.nodes[&target_decl_label.as_label()].downcast();

        // Look up the field
        target_decl.find_field(&self.nodes[&var_use.as_label()].downcast::<FieldAccess>().field_name)
                   .assert_as_var_decl()
    }

    pub fn query_method_decl(&mut self, method_use: labels::MethodUse) -> labels::MethodDecl {
        // Get the ClassDecl of the target
        let target_label = self.nodes[&method_use.as_label()].downcast::<MethodCall>().target.label();
        let is_static;
        let target_decl_label = match self.query_expr_type(target_label) {
            Some(target_ty) => {
                // Non-static method
                is_static = false;
                self.classes[&target_ty]
            }
            None => {
                // Static method
                // The target should be an Expression::Identifier
                is_static = true;
                let class_name = self.nodes[&target_label.as_label()].downcast::<Identifier>().name.to_owned();
                self.query_class_decl_by_name(&class_name)
            }
        };

        let target_decl: &ClassDecl = self.nodes[&target_decl_label.as_label()].downcast();

        // Look up the field
        target_decl.find_method(is_static, &self.nodes[&method_use.as_label()].downcast::<MethodCall>().method_name)
                   .assert_as_method_decl()
    }

    pub fn query_param_types(&mut self, method: labels::MethodDecl) -> Vec<TypeId> {
        let md: &MethodDecl = self.nodes[&method.as_label()].downcast();
        unimplemented!()
        //md.params.iter().map(|p| self.types.get_id(p.ty)).collect()
    }

    pub fn query_constructor(&mut self, class: labels::ClassDecl) -> labels::MethodDecl {
        let cd: &ClassDecl = self.nodes[&class.as_label()].downcast();
        unimplemented!()
    }

    pub fn query_class_decl(&mut self, class_use: labels::TypeUse) -> labels::ClassDecl {
        // Search in our class map. If not present, search through all class declarations
        unimplemented!()
    }

    pub fn query_class_decl_by_name(&mut self, name: &str) -> labels::ClassDecl {
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
        self.nodes[&method.as_label()].downcast::<MethodDecl>().is_static
    }

    /// Returns the type of an expression.
    ///
    /// Note: references to undefined variables have no type.
    pub fn query_expr_type(&mut self, expr: labels::Expression) -> Option<TypeId> {
        unimplemented!()
    }
}
