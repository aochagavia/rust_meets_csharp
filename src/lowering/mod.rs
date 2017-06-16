use std::mem;

use analysis::ClassInfo;
use ast;
use ir;

pub struct ProgramMetadata {
    pub classes: Vec<ClassInfo>,
}

pub struct LoweringContext {
    pub metadata: ProgramMetadata,
    pub methods: Vec<ir::Method>
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
                    let var_id = unimplemented!();
                    let value = self.lower_expression(&assign.expr);
                    lowered_body.push(ir::Statement::Assign(ir::Assign { var_id, value }));
                }
                ast::Statement::Expression(ref expr) => {
                    let expr = self.lower_expression(expr);
                    lowered_body.push(ir::Statement::Expression(expr));
                }
                ast::Statement::Return(ref ret) => {
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
                let left = self.lower_expression(&bin_op.left);
                let right = self.lower_expression(&bin_op.right);
                ir::Expression::Intrinsic(Box::new(
                    ir::Intrinsic::IntOp(bin_op.operator, left, right)))
            }
            ast::Expression::FieldAccess(ref fa) => {
                // FIXME: type information of the target required
                // FIXME: mapping from field names to field_id in class
                let target = self.lower_expression(&fa.target);
                let field_id = unimplemented!();
                ir::Expression::FieldAccess(Box::new(ir::FieldAccess { target, field_id }))
            }
            ast::Expression::Literal(ref l) => {
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
                // FIXME: method resolution
                // FIXME: handle non-static method calls as well
                let method_id = unimplemented!();
                self.lower_static_method_call(method_id, &mc.args);
            }
            ast::Expression::New(ref n) => {
                // FIXME: class and method resolution
                // The class name needs to be mapped to a class_id
                // We need to obtain the method_id associated to the constructor
                let class_id = unimplemented!();
                let constructor_id = unimplemented!();
                self.lower_method_call(ir::Expression::NewObject(class_id), constructor_id, &n.args);
            }
            ast::Expression::Identifier(ref i) => {
                // FIXME: name resolution
                let var_id = unimplemented!();
                ir::Expression::VarRead(var_id);
            }
            ast::Expression::This => {
                // When used from a method, the first parameter will always be this
                ir::Expression::VarRead(0)
            }
        }
    }

    fn lower_method_call(&mut self, target: ir::Expression, method_id: usize, args: &[ast::Expression]) -> ir::Expression {
        // Note: we need to pass the target expression as a first argument to the function
        let mut arguments = vec![target];
        arguments.extend(args.iter().map(|a| self.lower_expression(a)));

        ir::Expression::MethodCall(ir::MethodCall {
            method_id,
            arguments
        })
    }

    fn lower_static_method_call(&mut self, method_id: usize, args: &[ast::Expression]) -> ir::Expression {
        let arguments = args.iter().map(|a| self.lower_expression(a)).collect();

        ir::Expression::MethodCall(ir::MethodCall {
            method_id,
            arguments
        })
    }
}
