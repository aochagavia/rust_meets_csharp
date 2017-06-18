use std::mem;

use analysis::{self, ClassInfo, MethodId, VarId, QueryEngine};
use ast;
use ir;

pub struct ProgramMetadata {
    pub classes: Vec<ClassInfo>,
}

pub struct LoweringContext {
    pub metadata: ProgramMetadata,
    pub methods: Vec<ir::Method>,
    pub query_engine: QueryEngine
}

pub fn lower(p: &ast::Program) -> (ir::Program, ProgramMetadata) {
    // FIXME: implement real lowering
    // Note: we only want to lower a Program if it passes all checks...
    // When performing on-demand analysis, we need to ensure all code is valid, including dead code.

    let classes = vec![
        ClassInfo {
            superclass_id: None,
            name: "Console".to_string(),
            field_names: Vec::new(),
            methods: vec![0]
        }
    ];
    let entry_point = 0;
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

    (ir::Program { entry_point, methods }, ProgramMetadata { classes })
}

impl LoweringContext {
    fn lower_program(mut self, p: &ast::Program) -> ir::Program {
        // Lowering class declarations will automatically populate the metadata
        for &ast::TopItem::ClassDecl(ref cd) in &p.items {
            self.lower_class_decl(cd);
        }

        let entry_point = unimplemented!();

        ir::Program {
            entry_point,
            methods: mem::replace(&mut self.methods, Vec::new())
        }
    }

    fn lower_class_decl(&mut self, c: &ast::ClassDecl) {
        // Query class methods
    }

    fn lower_method(&mut self, m: &ast::MethodDecl) {
        let param_count = if m.is_static {
            m.params.len()
        } else {
            // `this` is passed as a parameter
            m.params.len() + 1
        };

        let mut lowered_body = Vec::new();
        for stmt in &m.body {
            match *stmt {
                ast::Statement::Assign(ref assign) => {
                    // We need name resolution information here
                    // Which id does this variable have?
                    // assign.var_name needs to be mapped to an id in the function...
                    // In other words, each function needs a HashMap<Label, usize>
                    // ?
                    // If we know this, we can proceed
                    // FIXME: check that assign.expr has the same type of the variable
                    let var_id = unimplemented!();
                    let value = self.lower_expression(&assign.expr);
                    lowered_body.push(ir::Statement::Assign(ir::Assign { var_id, value }));
                }
                ast::Statement::Expression(ref expr) => {
                    let expr = self.lower_expression(expr);
                    lowered_body.push(ir::Statement::Expression(expr));
                }
                ast::Statement::Return(ref ret) => {
                    // FIXME: check that the return value matches the function's return type
                    let expr = ret.expr.as_ref().map(|r| self.lower_expression(r));
                    lowered_body.push(ir::Statement::Return(expr));
                }
                ast::Statement::VarDecl(ref var_decl) => {
                    lowered_body.push(ir::Statement::VarDecl);

                    // Again, we need name resolution information if we want to write to the variable
                    // We should be able to get the var_id from the same HashMap<Label, usize>
                    let var_id = unimplemented!();
                    let value = var_decl.expr.as_ref().map(|e| self.lower_expression(e));
                    if let Some(value) = value {
                        lowered_body.push(ir::Statement::Assign(ir::Assign { var_id, value }))
                    }
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
                    analysis::Type::Class(id) => self.query_engine.query_field(id, &fa.field_name.name),
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
                let target_ty = self.query_engine.query_expr_type(mc.target.label()).cloned();
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
                // TODO: error reporting when class doesn't exist
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
            ast::Expression::This(_) => {
                // When used from a method, the first parameter will always be this
                ir::Expression::VarRead(VarId::this())
            }
        }
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
