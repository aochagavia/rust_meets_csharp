use std::collections::HashMap;
use std::mem;

use analysis::labels;
use ast::*;
use ast::visitor::Visitor;

#[derive(Debug)]
pub enum PreprocessError {
    MultiClassDecl(labels::ClassDecl),
    MultiEntryPoint(labels::MethodDecl)
}

// A visitor to collect class names
#[derive(Default)]
pub struct PreprocessVisitor<'a> {
    pub nodes: HashMap<Label, Node<'a>>,
    pub classes_by_name: HashMap<&'a str, &'a ClassDecl>,
    pub entry_point: Option<&'a MethodDecl>,
    pub errors: Vec<PreprocessError>
}

impl<'a> PreprocessVisitor<'a> {
    fn insert_node(&mut self, label: Label, node: Node<'a>) {
        let repeated = self.nodes.insert(label, node).is_some();
        assert_eq!(repeated, false, "Node labels should be unique");
    }
}

impl<'a> Visitor<'a> for PreprocessVisitor<'a> {
    fn visit_class_decl(&mut self, decl: &'a ClassDecl) {
        let label = decl.label.assert_as_class_decl();

        // Insert and check whether the class already exists
        let repeated = self.classes_by_name.insert(&decl.name, decl).is_some();
        if repeated {
            self.errors.push(PreprocessError::MultiClassDecl(label));
        }

        // Track nodes
        self.insert_node(decl.label, Node::ClassDecl(&decl));

        visitor::walk_class_decl(self, decl);
    }

    fn visit_field_decl(&mut self, decl: &'a FieldDecl) {
        // Track nodes
        self.insert_node(decl.label, Node::FieldDecl(&decl));

        visitor::walk_field_decl(self, decl)
    }

    fn visit_method_decl(&mut self, decl: &'a MethodDecl) {
        let label = decl.label.assert_as_method_decl();

        // Entry points must be static and be called Main. Parameters are ignored
        if decl.is_static && decl.name == "Main" {
            let repeated = mem::replace(&mut self.entry_point, Some(decl)).is_some();
            if repeated {
                self.errors.push(PreprocessError::MultiEntryPoint(label));
            }
        }

        // Track nodes
        self.insert_node(decl.label, Node::MethodDecl(&decl));

        visitor::walk_method_decl(self, decl);
    }

    fn visit_param(&mut self, param: &'a Param) {
        visitor::walk_param(self, param)
    }

    fn visit_statement(&mut self, statement: &'a Statement) {
        visitor::walk_statement(self, statement)
    }

    fn visit_assign(&mut self, assign: &'a Assign) {
        visitor::walk_assign(self, assign)
    }

    fn visit_expression(&mut self, expr: &'a Expression) {
        visitor::walk_expression(self, expr)
    }

    fn visit_return(&mut self, ret: &'a Return) {
        visitor::walk_return(self, ret)
    }

    fn visit_var_decl(&mut self, var_decl: &'a VarDecl) {
        visitor::walk_var_decl(self, var_decl)
    }

    fn visit_binary_op(&mut self, binary_op: &'a BinaryOp) {
        visitor::walk_binary_op(self, binary_op)
    }

    fn visit_field_access(&mut self, field_access: &'a FieldAccess) {
        // Track nodes
        self.insert_node(field_access.label, Node::FieldAccess(&field_access));

        visitor::walk_field_access(self, field_access)
    }

    fn visit_literal(&mut self, literal: &'a Literal) {
        visitor::walk_literal(self, literal)
    }

    fn visit_method_call(&mut self, method_call: &'a MethodCall) {
        // Track nodes
        self.insert_node(method_call.label, Node::MethodCall(&method_call));

        visitor::walk_method_call(self, method_call)
    }

    fn visit_new(&mut self, new: &'a New) {
        visitor::walk_new(self, new)
    }

    fn visit_identifier(&mut self, identifier: &'a Identifier) {
        // Track nodes
        self.insert_node(identifier.label, Node::Identifier(&identifier));

        visitor::walk_identifier(self, identifier)
    }

    fn visit_this(&mut self) {
        visitor::walk_this(self)
    }
}
