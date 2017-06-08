use ast::*;

#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
        walk_class_decl(self, class_decl)
    }

    fn visit_class_item(&mut self, class_item: &ClassItem) {
        walk_class_item(self, class_item)
    }

    fn visit_field_decl(&mut self, field_decl: &FieldDecl) {
        walk_field_decl(self, field_decl)
    }

    fn visit_method_decl(&mut self, method_decl: &MethodDecl) {
        walk_method_decl(self, method_decl)
    }

    fn visit_statement(&mut self, statement: &Statement) {
        walk_statement(self, statement)
    }

    fn visit_assign(&mut self, assign: &Assign) {
        walk_assign(self, assign)
    }

    fn visit_expression(&mut self, expr: &Expression) {
        walk_expression(self, expr)
    }

    fn visit_return(&mut self, ret: &Return) {
        walk_return(self, ret)
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        walk_var_decl(self, var_decl)
    }

    fn visit_binary_op(&mut self, binary_op: &BinaryOp) {
        walk_binary_op(self, binary_op)
    }

    fn visit_field_access(&mut self, field_access: &FieldAccess) {
        walk_field_access(self, field_access)
    }

    fn visit_literal(&mut self, literal: &Literal) {
        walk_literal(self, literal)
    }

    fn visit_method_call(&mut self, method_call: &MethodCall) {
        walk_method_call(self, method_call)
    }

    fn visit_new(&mut self, new: &New) {
        walk_new(self, new)
    }

    fn visit_var_read(&mut self, var_read: &str) {
        walk_var_read(self, var_read)
    }
}

pub fn walk_class_decl<V: Visitor>(visitor: &mut V, class_decl: &ClassDecl) {
    for item in &class_decl.items {
        visitor.visit_class_item(item);
    }
}

pub fn walk_class_item<V: Visitor>(visitor: &mut V, class_item: &ClassItem) {
    match *class_item {
        ClassItem::FieldDecl(ref fd) => visitor.visit_field_decl(fd),
        ClassItem::MethodDecl(ref md) => visitor.visit_method_decl(md)
    }
}

pub fn walk_field_decl<V: Visitor>(visitor: &mut V, field_decl: &FieldDecl) {

}

pub fn walk_method_decl<V: Visitor>(visitor: &mut V, method_decl: &MethodDecl) {

}

pub fn walk_statement<V: Visitor>(visitor: &mut V, statement: &Statement) {

}

pub fn walk_assign<V: Visitor>(visitor: &mut V, assign: &Assign) {

}

pub fn walk_expression<V: Visitor>(visitor: &mut V, expr: &Expression) {

}

pub fn walk_return<V: Visitor>(visitor: &mut V, ret: &Return) {

}

pub fn walk_var_decl<V: Visitor>(visitor: &mut V, var_decl: &VarDecl) {

}

pub fn walk_binary_op<V: Visitor>(visitor: &mut V, binary_op: &BinaryOp) {

}

pub fn walk_field_access<V: Visitor>(visitor: &mut V, field_access: &FieldAccess) {

}

pub fn walk_literal<V: Visitor>(visitor: &mut V, literal: &Literal) {

}

pub fn walk_method_call<V: Visitor>(visitor: &mut V, method_call: &MethodCall) {

}

pub fn walk_new<V: Visitor>(visitor: &mut V, new: &New) {

}

pub fn walk_var_read<V: Visitor>(visitor: &mut V, var_read: &str) {

}
