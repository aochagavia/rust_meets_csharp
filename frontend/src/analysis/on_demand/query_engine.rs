use std::collections::HashMap;

use analysis::{self, labels, AstPreprocessor, TypeId};
use ast::*;
use super::type_map::TypeMap;

pub struct QueryEngine<'a> {
    pub nodes: HashMap<Label, Node<'a>>,
    var_map: HashMap<Label, &'a VarDecl>,
    this_map: HashMap<Label, &'a ClassDecl>,
    types: TypeMap,
    classes: HashMap<labels::ClassDecl, &'a ClassDecl>,
    classes_by_name: HashMap<&'a str, &'a ClassDecl>,
    entry_point: &'a MethodDecl
}

#[allow(unused_variables)]
impl<'a> QueryEngine<'a> {
    pub fn new(program: &'a Program) -> QueryEngine<'a> {
        // FIXME: manually add type information for Console.WriteLine
        // We need a fake class called Console with a fake method called WriteLine
        let ast_data = AstPreprocessor::preprocess(program);

        QueryEngine {
            types: TypeMap::default(),
            nodes: ast_data.nodes,
            var_map: ast_data.var_map,
            this_map: ast_data.this_map,
            classes: HashMap::new(),
            classes_by_name: ast_data.classes_by_name,
            entry_point: ast_data.entry_point,
        }
    }

    pub fn entry_point(&self) -> &MethodDecl {
        &self.entry_point
    }

    pub fn types(&self) -> &TypeMap {
        &self.types
    }

    pub fn types_mut(&mut self) -> &mut TypeMap {
        &mut self.types
    }

    pub fn query_field(&mut self, var_use: labels::VarUse) -> labels::VarDecl {
        // Get the ClassDecl of the target
        let target_label = self.nodes[&var_use.as_label()].downcast::<FieldAccess>().target.label();
        // Note: a field target should always be an expression and have a type
        let target_ty = self.query_expr_type(target_label).unwrap();
        let decl_label = self.types.get(target_ty).class_decl();
        let target_decl = self.classes[&decl_label];

        // Look up the field
        target_decl.find_field(&self.nodes[&var_use.as_label()].downcast::<FieldAccess>().field_name)
                   .assert_as_var_decl()
    }

    pub fn query_method_decl(&mut self, method_use: labels::MethodUse) -> labels::MethodDecl {
        // We assume no queries about Console.WriteLine will ever be made

        // Get the ClassDecl of the target
        let target_label = self.nodes[&method_use.as_label()].downcast::<MethodCall>().target.label();
        let is_static;
        let decl_label = match self.query_expr_type(target_label) {
            Some(target_ty) => {
                // Non-static method
                is_static = false;
                self.types.get(target_ty).class_decl()
            }
            None => {
                // Static method
                // The target should be an Expression::Identifier, naming the type
                is_static = true;
                let class_name = self.nodes[&target_label.as_label()].downcast::<Identifier>().name.to_owned();
                self.query_class_decl(&class_name)
            }
        };

        let target_decl: &ClassDecl = self.nodes[&decl_label.as_label()].downcast();

        // Look up the field
        target_decl.find_method(is_static, &self.nodes[&method_use.as_label()].downcast::<MethodCall>().method_name)
                   .assert_as_method_decl()
    }

    pub fn query_param_types(&mut self, method: labels::MethodDecl) -> Vec<TypeId> {
        let md: &MethodDecl = self.nodes[&method.as_label()].downcast();
        let mut tys = Vec::new();
        for param in &md.params {
            let param_ty = self.types.get_from_ast_ty(&param.ty, &self.classes_by_name);
            tys.push(param_ty);
        }
        tys
    }

    pub fn query_class_decl(&mut self, name: &str) -> labels::ClassDecl {
        self.classes_by_name.get(name).expect("No class decl exist for given class name").label.assert_as_class_decl()
    }

    pub fn query_var_decl(&mut self, label: Label) -> labels::VarDecl {
        // The label may correspong to a VarAssign, VarDecl or Identifier
        if let Node::VarDecl(_) = self.nodes[&label] {
            return label.assert_as_var_decl();
        }

        // VarDecls and Identifiers are in the map
        self.var_map[&label].label.assert_as_var_decl()
    }

    pub fn query_var_type(&mut self, identifier: labels::VarDecl) -> TypeId {
        let vd: &VarDecl = self.nodes[&identifier.as_label()].downcast();
        self.types.get_from_ast_ty(&vd.ty, &self.classes_by_name)
    }

    pub fn query_return_type(&mut self, method: labels::MethodDecl) -> TypeId {
        let md: &MethodDecl = self.nodes[&method.as_label()].downcast();
        self.types.get_from_ast_ty(&md.return_ty, &self.classes_by_name)
    }

    pub fn query_is_static(&mut self, method: labels::MethodDecl) -> bool {
        self.nodes[&method.as_label()].downcast::<MethodDecl>().is_static
    }

    /// Returns the type of an expression.
    ///
    /// Note: references to undefined variables have no type.
    pub fn query_expr_type(&mut self, expr: labels::Expression) -> Option<TypeId> {
        // Here we go with the type checker!
        match self.nodes[&expr.as_label()] {
            Node::FieldAccess(fa) => {
                // Get the type of the target
                let target_ty = self.query_expr_type(fa.target.label()).expect("Target of field access has no type");
                // Go to the class, find the field declaration and return its type
                match self.types.get(target_ty) {
                    analysis::Type::Class(cd) => {
                        let class_decl: &ClassDecl = self.nodes[&cd.as_label()].downcast();
                        let field = class_decl.find_field(&fa.field_name);
                        let field_decl: &FieldDecl = self.nodes[&field].downcast();
                        Some(self.types.get_from_ast_ty(&field_decl.ty, &self.classes_by_name))
                    }
                    x => {
                        panic!("Attempted to access a field of something that is not a class: {:?}", x)
                    }
                }
            }
            Node::MethodCall(mc) => {
                // Built in Console.WriteLine
                if mc.is_console_write_line() {
                    return Some(self.types.void_ty());
                }

                // Get class decl of target
                let target_ty = self.query_expr_type(mc.target.label());
                let class_decl = match target_ty {
                    Some(ty) => {
                        // Non-static method
                        let decl_label = self.types.get(ty).class_decl();
                        self.classes[&decl_label]
                    }
                    None => {
                        // Static method
                        let class: &str = &mc.target.identifier().name;
                        self.classes_by_name[class]
                    }
                };

                // Find the method
                let method_decl = class_decl.find_method_any(&mc.method_name);

                // Collect parameter types
                let mut param_tys = Vec::new();
                for param in &method_decl.params {
                    let ty = self.types.get_from_ast_ty(&param.ty, &self.classes_by_name);
                    param_tys.push(ty);
                }

                // Collect arg types
                let mut arg_tys = Vec::new();
                for arg in &mc.args {
                    let ty = self.query_expr_type(arg.label()).expect("Unable to get type of method argument");
                    arg_tys.push(ty);
                }

                // Check length and unification of types
                if param_tys.len() != arg_tys.len() {
                    panic!("Mismatched param and arg length in method call");
                }
                for (&param_ty, &arg_ty) in param_tys.iter().zip(arg_tys.iter()) {
                    if !self.types.unify(param_ty, arg_ty) {
                        panic!("Mismatched types in method call arguments");
                    }
                }

                // The type of the method call is the return type of the method decl
                Some(self.types.get_from_ast_ty(&method_decl.return_ty, &self.classes_by_name))
            }
            Node::Identifier(i) => {
                // Get the var decl associated to this identifier and return its type
                // Note: it is possible that the identifier refers to a class name. In that case we return None.
                match self.var_map.get(&i.label) {
                    Some(var_decl) => {
                        Some(self.types.get_from_ast_ty(&var_decl.ty, &self.classes_by_name))
                    }
                    None => {
                        None
                    }
                }
            }
            Node::BinaryOp(bo) => {
                let left_ty = self.query_expr_type(bo.left.label()).expect("No type found for lhs of binary op");
                let right_ty = self.query_expr_type(bo.right.label()).expect("No type found for rhs of binary op");
                let well_typed = left_ty == right_ty && left_ty == self.types.int_ty();
                if !well_typed {
                    panic!("Mismatched types in binary operation");
                }
                // We only support int operations
                Some(self.types.int_ty())
            }
            Node::Literal(l) => {
                match &l.kind {
                    &LiteralKind::Int(_) => {
                        Some(self.types.int_ty())
                    }
                    &LiteralKind::Null => {
                        Some(self.types.any_ty())
                    }
                    &LiteralKind::String(_) => {
                        Some(self.types.string_ty())
                    }
                    &LiteralKind::Array(ref ast_ty, _) => {
                        let inner_ty = self.types.get_from_ast_ty(ast_ty, &self.classes_by_name);
                        Some(self.types.get_id(analysis::Type::Array(inner_ty)))
                    }
                }
            }
            Node::New(n) => {
                Some(self.types.get_from_class_name(&n.class_name, &self.classes_by_name))
            }
            Node::This(t) => {
                let class_decl = self.this_map[&t.label];
                Some(self.types.get_from_class_name(&class_decl.name, &self.classes_by_name))
            }
            // Not an expression
            _ => {
                panic!("Called query_expr_type on an AST node that is not an expression");
            }
        }
    }
}
