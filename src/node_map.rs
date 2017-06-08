use std::collections::HashMap;

use ast::*;
use visitor::{self, Visitor};

// Note that copying a Node results in a shallow copy
#[derive(Copy, Clone, Debug)]
pub enum Node<'a> {
    ClassDecl(&'a ClassDecl),
    FieldDecl(&'a FieldDecl),
    MethodDecl(&'a MethodDecl),
    Param(&'a Param),
    Assign(&'a Assign),
    VarDecl(&'a VarDecl),
    FieldAccess(&'a FieldAccess),
    MethodCall(&'a MethodCall),
    New(&'a New),
}

// A map from node labels to nodes
#[derive(Debug)]
pub struct NodeMap<'a> {
    map: HashMap<Label, Node<'a>>
}

impl<'a> NodeMap<'a> {
    pub fn build(p: &'a Program) -> NodeMap<'a> {
        let mut visitor = NodeMapVisitor { map: HashMap::new() };
        visitor.visit_program(p);
        NodeMap { map: visitor.map }
    }

    pub fn get_node(&self, label: Label) -> Option<Node<'a>> {
        self.map.get(&label).cloned()
    }
}

// Visit all nodes that have a label
struct NodeMapVisitor<'a> {
    map: HashMap<Label, Node<'a>>
}

impl<'a> NodeMapVisitor<'a> {
    fn insert<T, F>(&mut self, label: Label, thing: &'a T, transform: F)
    where
        F: Fn(&'a T) -> Node<'a>
    {
        let existing_val = self.map.insert(label, transform(thing));
        assert!(existing_val.is_none());
    }
}

impl<'a> Visitor<'a> for NodeMapVisitor<'a> {
    fn visit_class_decl(&mut self, decl: &'a ClassDecl) {
        self.insert(decl.label, decl, Node::ClassDecl);
        visitor::walk_class_decl(self, decl)
    }

    fn visit_field_decl(&mut self, decl: &'a FieldDecl) {
        self.insert(decl.label, decl, Node::FieldDecl);
        visitor::walk_field_decl(self, decl)
    }

    fn visit_method_decl(&mut self, decl: &'a MethodDecl) {
        self.insert(decl.label, decl, Node::MethodDecl);
        visitor::walk_method_decl(self, decl)
    }

    fn visit_param(&mut self, param: &'a Param) {
        self.insert(param.label, param, Node::Param);
        visitor::walk_param(self, param)
    }

    fn visit_assign(&mut self, assign: &'a Assign) {
        self.insert(assign.label, assign, Node::Assign);
        visitor::walk_assign(self, assign)
    }

    fn visit_var_decl(&mut self, decl: &'a VarDecl) {
        self.insert(decl.label, decl, Node::VarDecl);
        visitor::walk_var_decl(self, decl)
    }

    fn visit_field_access(&mut self, fa: &'a FieldAccess) {
        self.insert(fa.label, fa, Node::FieldAccess);
        visitor::walk_field_access(self, fa)
    }

    fn visit_method_call(&mut self, mc: &'a MethodCall) {
        self.insert(mc.label, mc, Node::MethodCall);
        visitor::walk_method_call(self, mc)
    }

    fn visit_new(&mut self, new: &'a New) {
        self.insert(new.label, new, Node::New);
        visitor::walk_new(self, new)
    }
}
