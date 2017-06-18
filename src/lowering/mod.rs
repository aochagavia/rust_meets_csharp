use analysis::{self, ClassInfo, MethodId, VarId, QueryEngine};
use ast;
use ir;

pub struct ProgramMetadata {
    pub classes: Vec<ClassInfo>,
    pub entry_point: MethodId
}

pub struct LoweringContext<'engine, 'a: 'engine> {
    pub query_engine: &'engine mut QueryEngine<'a>
}

/*
pub fn hello_world() -> ir::Program) {
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

impl<'a, 'engine> LoweringContext<'a, 'engine> {
    pub fn lower_program(mut self) -> ir::Program {
        let mut methods = Vec::new();

        // Generate code for intrinsics
        for intrinsic in QueryEngine::intrinsics() {
            methods.push(self.lower_intrinsic_method(intrinsic));
        }

        // Generate code for user defined methods
        for md in self.query_engine.methods() {
            methods.push(self.lower_method(md));
        }

        ir::Program { methods }
    }

    fn lower_intrinsic_method(&mut self, intrinsic: analysis::IntrinsicInfo) -> ir::Method {
        unimplemented!()
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

                let expr_ty = ret.expr.as_ref().map(|e| self.query_engine.query_expr_type(e.label()).unwrap())
                                                .unwrap_or(analysis::Type::Void);

                if ret_ty != expr_ty {
                    panic!("Type mismatch in return statement: {:?} and {:?}", ret_ty, expr_ty);
                }

                let expr = ret.expr.as_ref().map(|r| self.lower_expression(r));
                body.push(ir::Statement::Return(expr));
            }
            ast::Statement::VarDecl(ref var_decl) => {
                body.push(ir::Statement::VarDecl);
                if let Some(ref expr) = var_decl.expr {
                    body.push(self.lower_assignment(var_decl.label, expr));
                }
            }
        }
    }

    fn lower_expression(&mut self, e: &ast::Expression) -> ir::Expression {
        match *e {
            ast::Expression::BinaryOp(ref bin_op) => {
                // First, type check
                // Safe to unwrap, since we know that expressions always have a type
                let left_ty = self.query_engine.query_expr_type(bin_op.left.label()).unwrap().clone();
                let right_ty = self.query_engine.query_expr_type(bin_op.right.label()).unwrap().clone();

                match (left_ty, right_ty) {
                    (analysis::Type::Int, analysis::Type::Int) => (),
                    (l, r) => panic!("Invalid types in binary operator: {:?} and {:?}. Only int allowed!",
                                     l, r)
                }

                // Generate code
                let left = self.lower_expression(&bin_op.left);
                let right = self.lower_expression(&bin_op.right);
                ir::Expression::Intrinsic(Box::new(
                    ir::Intrinsic::IntOp(bin_op.operator, left, right)))
            }
            ast::Expression::FieldAccess(ref fa) => {
                // Query type and field id
                let target_ty = self.query_engine.query_expr_type(fa.target.label()).unwrap().clone();
                let field_id = match target_ty {
                    analysis::Type::Class(id) => self.query_engine.query_field(id, &fa.field_name),
                    ty => panic!("Invalid type in field access target: {:?}", ty)
                };

                // Generate code
                let target = self.lower_expression(&fa.target);
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
                let target_ty = self.query_engine.query_expr_type(mc.target.label());
                match target_ty {
                    Some(analysis::Type::Class(id)) => {
                        // FIXME: error reporting when method doesn't exist
                        // FIXME: check that the types match between arguments and parameters
                        let method_id = self.query_engine.query_method(id, &mc.method_name);
                        let expr = self.lower_expression(&mc.target);
                        self.lower_method_call(expr, method_id, &mc.args)
                    }
                    Some(ty) => panic!("Invalid type in method call target: {:?}", ty),
                    None => {
                        // The target has no type, which means that it is not an expression
                        // Therefore it should be a class name and this is a static method
                        let class_name = match *mc.target {
                            ast::Expression::Identifier(ref i) => &i.name,
                            _ => unreachable!()
                        };

                        match self.query_engine.query_class(class_name) {
                            Some(class_id) => {
                                // FIXME: error reporting when method doesn't exist
                                // FIXME: check that the types match between arguments and parameters
                                let method_id = self.query_engine.query_method(class_id, &mc.method_name);
                                self.lower_static_method_call(method_id, &mc.args)
                            }
                            None => panic!("Undeclared variable: {}", class_name)
                        }
                    }
                }
            }
            ast::Expression::New(ref n) => {
                // FIXME: error reporting when class doesn't exist
                let class_id = self.query_engine.query_class(&n.class_name).unwrap();
                // Note that a class always has a constructor
                let constructor_id = self.query_engine.query_constructor(class_id);

                // FIXME: check that the types match between arguments and parameters

                self.lower_method_call(ir::Expression::NewObject(class_id), constructor_id, &n.args)
            }
            ast::Expression::Identifier(ref i) => {
                let var_id = self.query_engine.query_var(i.label);
                ir::Expression::VarRead(var_id)
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
        // Note: the only assignable things are variables. No array indexing.
        let var_id = self.query_engine.query_var_decl(target);
        let var_ty = self.query_engine.query_var_type(var_id).clone();
        let expr_ty = self.query_engine.query_expr_type(expr.label()).unwrap().clone();
        if var_ty != expr_ty {
            panic!("Type mismatch in assignment: {:?} and {:?}", var_ty, expr_ty);
        }

        let value = self.lower_expression(expr);
        ir::Statement::Assign(ir::Assign { var_id, value })
    }

    fn lower_method_call(&mut self, target: ir::Expression, method_id: MethodId, args: &[ast::Expression]) -> ir::Expression {
        // Note: we need to pass the target expression as a first argument to the function
        let mut arguments = vec![target];
        arguments.extend(args.iter().map(|a| self.lower_expression(a)));

        ir::Expression::MethodCall(ir::MethodCall {
            method_id,
            arguments
        })
    }

    fn lower_static_method_call(&mut self, method_id: MethodId, args: &[ast::Expression]) -> ir::Expression {
        let arguments = args.iter().map(|a| self.lower_expression(a)).collect();

        ir::Expression::MethodCall(ir::MethodCall {
            method_id,
            arguments
        })
    }
}
