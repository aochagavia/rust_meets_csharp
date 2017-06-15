use ast::*;

#[allow(unused_variables)]
pub trait Visitor<'a>: Sized {
    fn visit_program(&mut self, program: &'a Program) {
        walk_program(self, program)
    }

    fn visit_class_decl(&mut self, class_decl: &'a ClassDecl) {
        walk_class_decl(self, class_decl)
    }

    fn visit_class_item(&mut self, class_item: &'a ClassItem) {
        walk_class_item(self, class_item)
    }

    fn visit_field_decl(&mut self, field_decl: &'a FieldDecl) {
        walk_field_decl(self, field_decl)
    }

    fn visit_method_decl(&mut self, method_decl: &'a MethodDecl) {
        walk_method_decl(self, method_decl)
    }

    fn visit_param(&mut self, param: &'a Param) {
        walk_param(self, param)
    }

    fn visit_statement(&mut self, statement: &'a Statement) {
        walk_statement(self, statement)
    }

    fn visit_assign(&mut self, assign: &'a Assign) {
        walk_assign(self, assign)
    }

    fn visit_expression(&mut self, expr: &'a Expression) {
        walk_expression(self, expr)
    }

    fn visit_return(&mut self, ret: &'a Return) {
        walk_return(self, ret)
    }

    fn visit_var_decl(&mut self, var_decl: &'a VarDecl) {
        walk_var_decl(self, var_decl)
    }

    fn visit_binary_op(&mut self, binary_op: &'a BinaryOp) {
        walk_binary_op(self, binary_op)
    }

    fn visit_field_access(&mut self, field_access: &'a FieldAccess) {
        walk_field_access(self, field_access)
    }

    fn visit_literal(&mut self, literal: &'a Literal) {
        walk_literal(self, literal)
    }

    fn visit_method_call(&mut self, method_call: &'a MethodCall) {
        walk_method_call(self, method_call)
    }

    fn visit_new(&mut self, new: &'a New) {
        walk_new(self, new)
    }

    fn visit_identifier(&mut self, identifier: &'a Identifier) {
        walk_identifier(self, identifier)
    }
}

pub fn walk_program<'a, V: Visitor<'a>>(visitor: &mut V, program: &'a Program) {
    for item in &program.items {
        let &TopItem::ClassDecl(ref cd) = item;
        visitor.visit_class_decl(cd);
    }
}

pub fn walk_class_decl<'a, V: Visitor<'a>>(visitor: &mut V, class_decl: &'a ClassDecl) {
    for item in &class_decl.items {
        visitor.visit_class_item(item);
    }
}

pub fn walk_class_item<'a, V: Visitor<'a>>(visitor: &mut V, class_item: &'a ClassItem) {
    match *class_item {
        ClassItem::FieldDecl(ref fd) => visitor.visit_field_decl(fd),
        ClassItem::MethodDecl(ref md) => visitor.visit_method_decl(md)
    }
}

pub fn walk_field_decl<'a, V: Visitor<'a>>(visitor: &mut V, field_decl: &'a FieldDecl) {
    if let Some(ref expr) = field_decl.assignment {
        visitor.visit_expression(expr);
    }
}

pub fn walk_method_decl<'a, V: Visitor<'a>>(visitor: &mut V, method_decl: &'a MethodDecl) {
    for param in &method_decl.params {
        visitor.visit_param(param);
    }

    for statement in &method_decl.body {
        visitor.visit_statement(statement);
    }
}

pub fn walk_param<'a, V: Visitor<'a>>(visitor: &mut V, param: &'a Param) { }

pub fn walk_statement<'a, V: Visitor<'a>>(visitor: &mut V, statement: &'a Statement) {
    match *statement {
        Statement::Assign(ref a) => visitor.visit_assign(a),
        Statement::Expression(ref e) => visitor.visit_expression(e),
        Statement::Return(ref r) => visitor.visit_return(r),
        Statement::VarDecl(ref vd) => visitor.visit_var_decl(vd)
    }
}

pub fn walk_assign<'a, V: Visitor<'a>>(visitor: &mut V, assign: &'a Assign) {
    visitor.visit_identifier(&assign.var_name);
    visitor.visit_expression(&assign.expr)
}

pub fn walk_return<'a, V: Visitor<'a>>(visitor: &mut V, ret: &'a Return) {
    if let Some(ref expr) = ret.expr {
        visitor.visit_expression(expr);
    }
}

pub fn walk_var_decl<'a, V: Visitor<'a>>(visitor: &mut V, var_decl: &'a VarDecl) {
    visitor.visit_identifier(&var_decl.var_name);
    if let Some(ref expr) = var_decl.expr {
        visitor.visit_expression(expr);
    }
}

pub fn walk_expression<'a, V: Visitor<'a>>(visitor: &mut V, expr: &'a Expression) {
    match *expr {
        Expression::BinaryOp(ref op) => visitor.visit_binary_op(op),
        Expression::FieldAccess(ref fa) => visitor.visit_field_access(fa),
        Expression::Literal(ref l) => visitor.visit_literal(l),
        Expression::MethodCall(ref mc) => visitor.visit_method_call(mc),
        Expression::New(ref n) => visitor.visit_new(n),
        Expression::Identifier(ref i) => visitor.visit_identifier(i)
    }
}

pub fn walk_binary_op<'a, V: Visitor<'a>>(visitor: &mut V, binary_op: &'a BinaryOp) {
    visitor.visit_expression(&binary_op.left);
    visitor.visit_expression(&binary_op.right);
}

pub fn walk_field_access<'a, V: Visitor<'a>>(visitor: &mut V, field_access: &'a FieldAccess) {
    visitor.visit_identifier(&field_access.var_name);
    visitor.visit_identifier(&field_access.field_name);
}

pub fn walk_literal<'a, V: Visitor<'a>>(visitor: &mut V, literal: &'a Literal) { }

pub fn walk_method_call<'a, V: Visitor<'a>>(visitor: &mut V, method_call: &'a MethodCall) {
    visitor.visit_expression(&method_call.target);
    for arg in &method_call.args {
        visitor.visit_expression(arg);
    }
}

pub fn walk_new<'a, V: Visitor<'a>>(visitor: &mut V, new: &'a New) {
    for arg in &new.args {
        visitor.visit_expression(arg);
    }
}

pub fn walk_identifier<'a, V: Visitor<'a>>(visitor: &mut V, identifier: &'a Identifier) { }
