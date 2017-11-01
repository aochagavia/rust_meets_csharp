extern crate frontend;

use std::collections::HashMap;
use frontend::analysis::{self, AstPreprocessor, Type, TypeId, TypeMap};
use frontend::ast::*;
use frontend::ast::visitor::Visitor;

pub fn check(program: &Program) -> (HashMap<Label, TypeId>, TypeMap) {
    let results = AstPreprocessor::preprocess(program);
    let mut visitor = TypeckVisitor {
        class_map: results.classes_by_name,
        node_map: results.nodes,
        var_map: results.var_map,
        this_map: results.this_map,
        output: HashMap::new(),
        types: TypeMap::default()
    };

    visitor.visit_ast(&program.items);
    (visitor.output, visitor.types)
}

struct TypeckVisitor<'a> {
    pub class_map: HashMap<&'a str, &'a ClassDecl>,
    pub node_map: HashMap<Label, Node<'a>>,
    pub var_map: HashMap<Label, &'a VarDecl>,
    pub this_map: HashMap<Label, &'a ClassDecl>,
    pub output: HashMap<Label, TypeId>,
    pub types: TypeMap
}

impl<'a> Visitor<'a> for TypeckVisitor<'a> {
    fn visit_expression(&mut self, expr: &'a Expression) {
        visitor::walk_expression(self, expr);

        match *expr {
            Expression::FieldAccess(ref fa) => {
                // Get the type of the target (we assume the type is already known)
                let target_ty = *self.output.get(&fa.target.label().as_label()).expect("Target of field access has no type");

                // Go to the class, find the field declaration and return its type
                match self.types.get(target_ty) {
                    Type::Class(cd) => {
                        let class_decl: &ClassDecl = self.node_map[&cd.as_label()].downcast();
                        let field = class_decl.find_field(&fa.field_name);
                        let field_decl: &FieldDecl = self.node_map[&field].downcast();

                        // Save the type to the table
                        self.output.insert(fa.label, self.types.get_from_ast_ty(&field_decl.ty, &self.class_map));
                    }
                    x => {
                        panic!("Attempted to access a field of something that is not a class: {:?}", x)
                    }
                }
            }
            Expression::Literal(ref l) => {
                let ty = match &l.kind {
                    &LiteralKind::Bool(_) => {
                        self.types.bool_ty()
                    }
                    &LiteralKind::Int(_) => {
                        self.types.int_ty()
                    }
                    &LiteralKind::Null => {
                        self.types.any_ty()
                    }
                    &LiteralKind::String(_) => {
                        self.types.string_ty()
                    }
                    &LiteralKind::Array(ref ast_ty, _) => {
                        let inner_ty = self.types.get_from_ast_ty(ast_ty, &self.class_map);
                        self.types.get_id(analysis::Type::Array(inner_ty))
                    }
                };

                self.output.insert(l.label, ty);
            }
            Expression::MethodCall(ref mc) => {
                // Built in Console.WriteLine
                if mc.is_console_write_line() {
                    self.output.insert(mc.label, self.types.void_ty());
                    return;
                }

                // Get class decl of target
                let class_decl = match self.output.get(&mc.target.label().as_label()) {
                    Some(ty) => {
                        // Non-static method
                        let decl_label = self.types.get(*ty).class_decl();

                        // Stupid workaround the borrow checker
                        let class_name: &str = &self.node_map[&decl_label.as_label()].downcast::<ClassDecl>().name;
                        self.class_map[class_name]
                    }
                    None => {
                        // Static method
                        let class: &str = &mc.target.identifier().name;
                        self.class_map[class]
                    }
                };

                // Find the method
                let method_decl = class_decl.find_method_any(&mc.method_name);

                // Collect parameter types
                let mut param_tys = Vec::new();
                for param in &method_decl.params {
                    let ty = self.types.get_from_ast_ty(&param.ty, &self.class_map);
                    param_tys.push(ty);
                }

                // Collect arg types
                let mut arg_tys: Vec<TypeId> = Vec::new();
                for arg in &mc.args {
                    let ty = *self.output.get(&arg.label().as_label()).expect("Unable to get type of method argument");
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
                self.output.insert(mc.label, self.types.get_from_ast_ty(&method_decl.return_ty, &self.class_map));
            }
            Expression::Identifier(ref i) => {
                // Get the var decl associated to this identifier and return its type
                // Note: it is possible that the identifier refers to a class name. In that case we return None.
                if let Some(var_decl) = self.var_map.get(&i.label) {
                    self.output.insert(i.label, self.types.get_from_ast_ty(&var_decl.ty, &self.class_map));
                }
            }
            Expression::BinaryOp(ref bo) => {
                let left_ty = *self.output.get(&bo.left.label().as_label()).expect("No type found for lhs of binary op");
                let right_ty = *self.output.get(&bo.right.label().as_label()).expect("No type found for rhs of binary op");
                let well_typed = left_ty == right_ty && left_ty == self.types.int_ty();
                if !well_typed {
                    panic!("Mismatched types in binary operation");
                }
                // We only support int operations
                self.output.insert(bo.label, self.types.int_ty());
            }
            Expression::New(ref n) => {
                self.output.insert(n.label, self.types.get_from_class_name(&n.class_name, &self.class_map));
            }
            Expression::This(ref t) => {
                let class_decl = self.this_map[&t.label];
                self.output.insert(t.label, self.types.get_from_class_name(&class_decl.name, &self.class_map));
            }
        }
    }
}
