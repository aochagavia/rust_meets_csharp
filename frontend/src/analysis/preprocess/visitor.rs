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
    pub var_map: HashMap<Label, &'a VarDecl>,
    pub this_map: HashMap<Label, &'a ClassDecl>,
    pub classes_by_name: HashMap<&'a str, &'a ClassDecl>,
    pub entry_point: Option<&'a MethodDecl>,
    pub errors: Vec<PreprocessError>,

    // Used during processing
    pub current_class: Option<&'a ClassDecl>,
    pub current_vars: HashMap<&'a str, &'a VarDecl>,
}

impl<'a> PreprocessVisitor<'a> {
    fn insert_node(&mut self, label: Label, node: Node<'a>) {
        let repeated = self.nodes.insert(label, node).is_some();
        assert_eq!(repeated, false, "Node labels should be unique");
    }
}

impl<'a> Visitor<'a> for PreprocessVisitor<'a> {
    fn visit_class_decl(&mut self, decl: &'a ClassDecl) {
        // Necessary bookkeeping
        self.current_class = Some(decl);

        let label = decl.label.assert_as_class_decl();

        // Class map
        let repeated = self.classes_by_name.insert(&decl.name, decl).is_some();
        if repeated {
            self.errors.push(PreprocessError::MultiClassDecl(label));
        }

        // Node tracking
        self.insert_node(decl.label, Node::ClassDecl(&decl));

        visitor::walk_class_decl(self, decl);
    }

    fn visit_field_decl(&mut self, decl: &'a FieldDecl) {
        self.insert_node(decl.label, Node::FieldDecl(&decl));
        visitor::walk_field_decl(self, decl)
    }

    fn visit_method_decl(&mut self, decl: &'a MethodDecl) {
        // Necessary bookkeeping for name resolution
        self.current_vars.clear();

        let label = decl.label.assert_as_method_decl();

        // Entry points must be static and be called Main. Parameters are ignored
        if decl.is_static && decl.name == "Main" {
            let repeated = mem::replace(&mut self.entry_point, Some(decl)).is_some();
            if repeated {
                self.errors.push(PreprocessError::MultiEntryPoint(label));
            }
        }

        // Node tracking
        self.insert_node(decl.label, Node::MethodDecl(&decl));
        visitor::walk_method_decl(self, decl);

        println!("Method: {}. Declared vars: {:?}", decl.name, self.current_vars);
    }

    fn visit_statement(&mut self, statement: &'a Statement) {
        visitor::walk_statement(self, statement)
    }

    fn visit_assign(&mut self, assign: &'a Assign) {
        // Name resolution
        let name: &str = &assign.var_name;
        let decl = self.current_vars[name];
        self.var_map.insert(assign.label, decl);

        visitor::walk_assign(self, assign)
    }

    fn visit_expression(&mut self, expr: &'a Expression) {
        visitor::walk_expression(self, expr)
    }

    fn visit_return(&mut self, ret: &'a Return) {
        visitor::walk_return(self, ret)
    }

    fn visit_var_decl(&mut self, var_decl: &'a VarDecl) {
        // Var tracking for name resolution
        if let Some(_) = self.current_vars.insert(&var_decl.var_name, var_decl) {
            // A variable with this name already exists in scope
            panic!("Double declaration of variable: {}", var_decl.var_name);
        }

        // Node tracking
        self.insert_node(var_decl.label, Node::VarDecl(var_decl));
        visitor::walk_var_decl(self, var_decl)
    }

    fn visit_binary_op(&mut self, binary_op: &'a BinaryOp) {
        self.insert_node(binary_op.label, Node::BinaryOp(binary_op));
        visitor::walk_binary_op(self, binary_op)
    }

    fn visit_field_access(&mut self, field_access: &'a FieldAccess) {
        self.insert_node(field_access.label, Node::FieldAccess(&field_access));
        visitor::walk_field_access(self, field_access)
    }

    fn visit_literal(&mut self, literal: &'a Literal) {
        self.insert_node(literal.label, Node::Literal(literal));
        visitor::walk_literal(self, literal)
    }

    fn visit_method_call(&mut self, method_call: &'a MethodCall) {
        // Track nodes
        self.insert_node(method_call.label, Node::MethodCall(&method_call));

        visitor::walk_method_call(self, method_call)
    }

    fn visit_new(&mut self, new: &'a New) {
        self.insert_node(new.label, Node::New(new));
        visitor::walk_new(self, new)
    }

    fn visit_identifier(&mut self, identifier: &'a Identifier) {
        // An identifier can refer to a variable or a type. We ignore them in the second case
        let name: &str = &identifier.name;
        if let Some(vd) = self.current_vars.get(name) {
            self.var_map.insert(identifier.label, vd);
        }

        // Node tracking
        self.insert_node(identifier.label, Node::Identifier(&identifier));
        visitor::walk_identifier(self, identifier)
    }

    fn visit_this(&mut self, this: &'a This) {
        // This map
        self.this_map.insert(this.label, self.current_class.unwrap());

        // Node tracking
        self.insert_node(this.label, Node::This(this));
        visitor::walk_this(self)
    }
}
