use std::collections::HashMap;

use analysis::{labels, AstPreprocessor, TypeId};
use ast::*;
use super::type_map::TypeMap;

pub struct QueryEngine<'a> {
    pub nodes: HashMap<Label, Node<'a>>,
    //program: &'a Program,
    //types: TypeMap,
    classes: HashMap<TypeId, &'a ClassDecl>,
    classes_by_name: HashMap<&'a str, &'a ClassDecl>,
    entry_point: &'a MethodDecl
}

#[allow(unused_variables)]
impl<'a> QueryEngine<'a> {
    pub fn new(program: &'a Program) -> QueryEngine<'a> {
        // FIXME: manually add type information for Console.WriteLine
        let ast_data = AstPreprocessor::preprocess(program);

        QueryEngine {
            //program,
            //types: TypeMap::default(),
            nodes: ast_data.nodes,
            classes: HashMap::new(),
            classes_by_name: ast_data.classes_by_name,
            entry_point: ast_data.entry_point
        }
    }

    pub fn entry_point(&self) -> &MethodDecl {
        &self.entry_point
    }

    pub fn types(&self) -> &TypeMap {
        unimplemented!()
        //&self.types
    }

    pub fn types_mut(&mut self) -> &mut TypeMap {
        unimplemented!()
        //&mut self.types
    }

    pub fn query_field(&mut self, var_use: labels::VarUse) -> labels::VarDecl {
        // Get the ClassDecl of the target
        let target_label = self.nodes[&var_use.as_label()].downcast::<FieldAccess>().target.label();
        // Note: a field target should always be an expression and have a type
        let target_ty = self.query_expr_type(target_label).unwrap();
        let target_decl = self.classes[&target_ty];

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
                self.classes[&target_ty].label
            }
            None => {
                // Static method
                // The target should be an Expression::Identifier, naming the type
                is_static = true;
                let class_name = self.nodes[&target_label.as_label()].downcast::<Identifier>().name.to_owned();
                self.query_class_decl(&class_name).as_label()
            }
        };

        let target_decl: &ClassDecl = self.nodes[&target_decl_label].downcast();

        // Look up the field
        target_decl.find_method(is_static, &self.nodes[&method_use.as_label()].downcast::<MethodCall>().method_name)
                   .assert_as_method_decl()
    }

    pub fn query_param_types(&mut self, method: labels::MethodDecl) -> Vec<TypeId> {
        let md: &MethodDecl = self.nodes[&method.as_label()].downcast();
        unimplemented!()
        //md.params.iter().map(|p| self.types.get_id(p.ty)).collect()
    }

    pub fn query_class_decl(&mut self, name: &str) -> labels::ClassDecl {
        self.classes_by_name.get(name).expect("No class decl exist for given class name").label.assert_as_class_decl()
    }

    pub fn query_var_decl(&mut self, var_use: Label) -> labels::VarDecl {
        // The label may correspong to a VarAssign, VarDecl or VarRead
        unimplemented!()
    }

    pub fn query_var_type(&mut self, identifier: labels::VarDecl) -> TypeId {
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
