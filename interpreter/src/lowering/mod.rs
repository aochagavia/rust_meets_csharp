use std::collections::HashMap;

use frontend::analysis::{self, QueryEngine};
use frontend::ast::{self, Label};
use ir::{self, FieldId, MethodId, VarId};

pub struct ClassInfo {
    pub name: String,
    pub field_names: Vec<String>,
}

pub struct LoweringContext<'engine, 'ast: 'engine> {
    pub ast: &'ast ast::Program,
    pub query_engine: &'engine mut QueryEngine<'ast>,
    pub methods: HashMap<Label, MethodId>,
    pub fields: HashMap<Label, FieldId>,
    pub classes: HashMap<Label, ClassInfo>
}

pub struct LoweringOutput {
    pub program: ir::Program,
    pub classes: HashMap<Label, ClassInfo>
}

/*
pub fn hello_world() -> ir::Program {
    let methods = vec![
        ir::Method {
            body: vec![
                ir::Statement::Expression(
                    ir::Expression::Intrinsic(
                        Box::new(
                            ir::Intrinsic::PrintLine(
                                ir::Expression::Literal(
                                    ir::Literal::String("Hello world!".to_string())
                                )
                            )
                        )
                    )
                )
            ]
        }
    ];
}
*/

impl<'engine, 'ast: 'engine> LoweringContext<'engine, 'ast> {
    pub fn new(ast: &'ast ast::Program, query_engine: &'engine mut QueryEngine<'ast>) -> LoweringContext<'ast, 'engine> {
        LoweringContext {
            ast,
            query_engine,
            methods: HashMap::new(),
            fields: HashMap::new(),
            classes: HashMap::new()
        }
    }

    fn register_var_decl(&mut self, var_decl: Label) {

    }

    fn get_var_id(&mut self, var_use: Label) -> VarId {
        unimplemented!()
    }

    pub fn lower_program(mut self) -> LoweringOutput {
        let mut methods = Vec::new();

        // Generate code for intrinsics
        for intrinsic in QueryEngine::intrinsics() {
            let label = ast::fresh_label();
            let method_id = MethodId(self.methods.len());
            self.methods.insert(label, method_id);
            methods.push(self.lower_console_write_line(intrinsic));
        }

        // Note, in the future we could also generate code for prelude methods
        // i.e. int.Max

        // Assign an id to all methods and fields
        for cd in self.ast.classes() {
            let mut field_names = Vec::new();
            for ci in &cd.items {
                match *ci {
                    ast::ClassItem::FieldDecl(ref fd) => {
                        let field_id = field_names.len();
                        field_names.push(fd.name.to_owned());
                        self.fields.insert(fd.label, FieldId(field_id));
                    }
                    ast::ClassItem::MethodDecl(ref md) => {
                        let method_id = MethodId(self.methods.len());
                        self.methods.insert(md.label, method_id);
                    }
                }
            }
            self.classes.insert(cd.label, ClassInfo {
                name: cd.name.to_owned(),
                field_names
            });
        }

        // Generate code
        for md in self.ast.methods() {
            methods.push(self.lower_method(md));
        }

        let ep = self.query_engine.query_entry_point();
        let program = ir::Program { methods, entry_point: self.methods[&ep] };
        LoweringOutput {
            program,
            classes: self.classes
        }
    }

    fn lower_console_write_line(&mut self, _: analysis::IntrinsicInfo) -> ir::Method {
        // We assume that the arguments are correctly passed. This is enforced by the type checker
        ir::Method {
            body: vec![
                ir::Statement::Expression(
                    ir::Expression::Intrinsic(
                        Box::new(
                            ir::Intrinsic::PrintLine(
                                ir::Expression::VarRead(
                                    VarId(0)
                                )
                            )
                        )
                    )
                )
            ]
        }
    }

    fn lower_method(&mut self, m: &ast::MethodDecl) -> ir::Method {
        let mut body = Vec::new();
        for stmt in &m.body {
            self.lower_statement(stmt, &mut body);
        }

        ir::Method { body }
    }

    fn lower_statement(&mut self, s: &ast::Statement, body: &mut Vec<ir::Statement>) {
        match *s {
            ast::Statement::Assign(ref assign) => {
                body.push(self.lower_assignment(assign.label, &assign.expr));
            }
            ast::Statement::Expression(ref expr) => {
                let expr = self.lower_expression(expr);
                body.push(ir::Statement::Expression(expr));
            }
            ast::Statement::Return(ref ret) => {
                let parent_method = self.query_engine.query_parent_method(ret.label);
                let ret_ty = self.query_engine.query_return_type(parent_method);

                let void_id = self.query_engine.types_mut().get_id(analysis::Type::Void);
                let expr_ty = ret.expr.as_ref().map(|e| self.query_engine.query_expr_type(e.label()).unwrap())
                                               .unwrap_or(void_id);

                if ret_ty != expr_ty {
                    panic!("Type mismatch in return statement: {:?} and {:?}", ret_ty, expr_ty);
                }

                let expr = ret.expr.as_ref().map(|r| self.lower_expression(r));
                body.push(ir::Statement::Return(expr));
            }
            ast::Statement::VarDecl(ref var_decl) => {
                body.push(ir::Statement::VarDecl);
                self.register_var_decl(var_decl.label);
                if let Some(ref expr) = var_decl.expr {
                    body.push(self.lower_assignment(var_decl.label, expr));
                }
            }
        }
    }

    fn lower_expression(&mut self, e: &ast::Expression) -> ir::Expression {
        match *e {
            ast::Expression::BinaryOp(ref bin_op) => {
                // Type check left and right
                let left_ty = self.query_engine.query_expr_type(bin_op.left.label()).unwrap();
                let right_ty = self.query_engine.query_expr_type(bin_op.right.label()).unwrap();

                let type_correct = left_ty == right_ty && self.query_engine.types().get(left_ty) == analysis::Type::Int;
                if !type_correct {
                    panic!("Invalid types in binary operator: {:?} and {:?}. Only int allowed!", left_ty, right_ty)
                }

                // Generate code
                let left = self.lower_expression(&bin_op.left);
                let right = self.lower_expression(&bin_op.right);
                ir::Expression::Intrinsic(Box::new(
                    ir::Intrinsic::IntOp(bin_op.operator, left, right)))
            }
            ast::Expression::FieldAccess(ref fa) => {
                // Query target type
                let target_ty = match self.query_engine.query_expr_type(fa.target.label()) {
                    Some(target_ty) => target_ty,
                    None => {
                        // Note: in the future, support for static fields could be added here
                        panic!("Attempt to access field on undefined identifier: {:?}", fa.target)
                    }
                };

                // Query field for the given type (if it exists)
                let field_label = match self.query_engine.types().get(target_ty) {
                    analysis::Type::Class(label) => self.query_engine.query_field(label, &fa.field_name),
                    ty => panic!("Invalid type in field access target: {:?}", ty)
                };

                // Generate code
                let target = self.lower_expression(&fa.target);
                let field_id = self.fields[&field_label];
                ir::Expression::FieldAccess(Box::new(ir::FieldAccess { target, field_id }))
            }
            ast::Expression::Literal(_, ref l) => {
                ir::Expression::Literal(match *l {
                    ast::Literal::Int(i) => ir::Literal::Int(i),
                    ast::Literal::String(ref s) => ir::Literal::String(s.clone()),
                    ast::Literal::Array(_, ref exprs) => {
                        let exprs = exprs.iter().map(|e| self.lower_expression(e)).collect();
                        ir::Literal::Array(exprs)
                    },
                    ast::Literal::Null => ir::Literal::Null
                })
            }
            ast::Expression::MethodCall(ref mc) => {
                // Query type and method id
                match self.query_engine.query_expr_type(mc.target.label()) {
                    Some(target_ty) => {
                        // Non-static method call
                        match self.query_engine.types().get(target_ty) {
                            analysis::Type::Class(id) => {
                                match self.query_engine.query_method(id, &mc.method_name) {
                                    Some(method_id) => {
                                        let expr = self.lower_expression(&mc.target);
                                        self.lower_method_call(method_id, Some(expr), &mc.args)
                                    }
                                    None => panic!("Method doesn't exist: {}", mc.method_name)
                                }
                            }
                            ty => panic!("Method call target is not an object: {:?}", ty),
                        }
                    }
                    None => {
                        // The target has no type, which means that it is an undefined variable usage
                        // Therefore, assume it the target is a class name and the method is static
                        let class_name = match *mc.target {
                            ast::Expression::Identifier(ref i) => &i.name,
                            _ => unreachable!()
                        };

                        match self.query_engine.query_class(class_name) {
                            Some(class_id) => {
                                match self.query_engine.query_method(class_id, &mc.method_name) {
                                    Some(label) => self.lower_method_call(label, None, &mc.args),
                                    None => panic!("Method doesn't exist: {}", mc.method_name)
                                }
                            }
                            None => panic!("Undeclared variable: {}", class_name)
                        }
                    }
                }
            }
            ast::Expression::New(ref n) => {
                match self.query_engine.query_class(&n.class_name) {
                    Some(class_label) => {
                        // Note that a class always has a constructor
                        let constructor_id = self.query_engine.query_constructor(class_label);
                        self.lower_method_call(constructor_id, Some(ir::Expression::NewObject(class_label)), &n.args)
                    }
                    None => panic!("Class doesn't exist: {}. How could it ever have a constructor?", n.class_name)
                }

            }
            ast::Expression::Identifier(ref i) => {
                let var_label = self.query_engine.query_var(i.label);
                ir::Expression::VarRead(self.get_var_id(var_label))
            }
            ast::Expression::This(label) => {
                let parent = self.query_engine.query_parent_method(label);
                if self.query_engine.query_is_static(parent) {
                    panic!("`this` keyword used inside static method");
                }

                // When used from a method, the first parameter will always be this
                ir::Expression::VarRead(VarId::this())
            }
        }
    }

    fn lower_assignment(&mut self, target: ast::Label, expr: &ast::Expression) -> ir::Statement {
        // Note: right now, the only lvalues are variables. No array indexing.
        // Type check
        let decl_label = self.query_engine.query_var_decl(target);
        let var_ty = self.query_engine.query_var_type(decl_label);
        let expr_ty = self.query_engine.query_expr_type(expr.label()).unwrap();
        if var_ty != expr_ty {
            panic!("Type mismatch in assignment: {:?} and {:?}", var_ty, expr_ty);
        }

        // Generate code
        let value = self.lower_expression(expr);
        let var_id = self.get_var_id(decl_label);
        ir::Statement::Assign(ir::Assign { var_id, value })
    }

    fn lower_method_call(&mut self, method_label: Label, this: Option<ir::Expression>, args: &[ast::Expression]) -> ir::Expression {
        // Note: we need to pass the target expression as a first argument to the function
        let mut arguments: Vec<_> = this.into_iter().collect();

        // Note, that we skip the `this` param here since we already know it is type correct
        let params = self.query_engine.query_param_types(method_label).into_iter().skip(arguments.len());
        for (arg, param_ty) in args.iter().zip(params) {
            // Type check argument and param
            let arg_ty = self.query_engine.query_expr_type(arg.label()).expect("Unreachable");
            if arg_ty != param_ty {
                panic!("Mismatched types between argument and parameter: {:?} and {:?}", arg_ty, param_ty);
            }

            // Generate code
            arguments.push(self.lower_expression(arg));
        }

        ir::Expression::MethodCall(ir::MethodCall {
            method_id: self.methods[&method_label],
            arguments
        })
    }
}
