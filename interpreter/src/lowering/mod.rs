mod var_tracker;

use std::collections::HashMap;

use frontend::analysis::{self, QueryEngine};
use frontend::analysis::labels;
use frontend::ast;
use ir::{self, FieldId, MethodId, VarId};
use self::var_tracker::VarTracker;

pub struct ClassInfo {
    pub name: String,
    pub field_names: Vec<String>,
}

pub struct LoweringContext<'engine, 'ast: 'engine> {
    pub ast: &'ast ast::Program,
    pub query_engine: &'engine mut QueryEngine<'ast>,
    methods: HashMap<labels::MethodDecl, MethodId>,
    fields: HashMap<labels::VarDecl, FieldId>,
    classes: HashMap<labels::ClassDecl, ClassInfo>,
    var_tracker: VarTracker
}

pub struct LoweringOutput {
    pub program: ir::Program,
    pub classes: HashMap<labels::ClassDecl, ClassInfo>
}

impl<'engine, 'ast: 'engine> LoweringContext<'engine, 'ast> {
    pub fn new(ast: &'ast ast::Program, query_engine: &'engine mut QueryEngine<'ast>) -> LoweringContext<'ast, 'engine> {
        LoweringContext {
            ast,
            query_engine,
            methods: HashMap::new(),
            fields: HashMap::new(),
            classes: HashMap::new(),
            var_tracker: VarTracker::default()
        }
    }

    pub fn lower_program(mut self) -> LoweringOutput {
        let mut methods = Vec::new();

        // Generate code for Console.WriteLine
        let method_id = MethodId(self.methods.len());
        self.methods.insert(ast::fresh_label().assert_as_method_decl(), method_id);
        methods.push(self.lower_console_write_line());

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
                        self.fields.insert(fd.label.assert_as_var_decl(), FieldId(field_id));
                    }
                    ast::ClassItem::MethodDecl(ref md) => {
                        let method_id = MethodId(self.methods.len());
                        self.methods.insert(md.label.assert_as_method_decl(), method_id);
                    }
                }
            }

            self.classes.insert(cd.label.assert_as_class_decl(), ClassInfo {
                name: cd.name.to_owned(),
                field_names
            });
        }

        // Generate code
        for md in self.ast.methods() {
            methods.push(self.lower_method(md));
        }

        let ep = self.query_engine.entry_point().label.assert_as_method_decl();
        let program = ir::Program { methods, entry_point: self.methods[&ep] };
        LoweringOutput {
            program,
            classes: self.classes
        }
    }

    fn lower_console_write_line(&mut self) -> ir::Method {
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
        self.var_tracker.reset();

        // Track declared parameters
        for param in &m.params {
            self.var_tracker.var_decl(param.label.assert_as_var_decl());
        }

        let mut body = Vec::new();
        for stmt in &m.body {
            self.lower_statement(stmt, &mut body, &m);
        }

        ir::Method { body }
    }

    fn lower_statement(&mut self, s: &ast::Statement, body: &mut Vec<ir::Statement>, parent_method: &ast::MethodDecl) {
        match *s {
            ast::Statement::Assign(ref assign) => {
                body.push(self.lower_assignment(assign.label, &assign.expr, parent_method));
            }
            ast::Statement::Expression(ref expr) => {
                let expr = self.lower_expression(expr, parent_method);
                body.push(ir::Statement::Expression(expr));
            }
            ast::Statement::Return(ref ret) => {
                let ret_ty = self.query_engine.query_return_type(parent_method.label.assert_as_method_decl());

                let void_id = self.query_engine.types_mut().get_id(analysis::Type::Void);
                let expr_ty = ret.expr.as_ref().map(|e| self.query_engine.query_expr_type(e.label()).unwrap())
                                               .unwrap_or(void_id);

                if ret_ty != expr_ty {
                    panic!("Type mismatch in return statement: {:?} and {:?}", ret_ty, expr_ty);
                }

                let expr = ret.expr.as_ref().map(|r| self.lower_expression(r, parent_method));
                body.push(ir::Statement::Return(expr));
            }
            ast::Statement::VarDecl(ref var_decl) => {
                // Track declared variables
                self.var_tracker.var_decl(var_decl.label.assert_as_var_decl());

                // Generate code
                body.push(ir::Statement::VarDecl);
                if let Some(ref expr) = var_decl.expr {
                    body.push(self.lower_assignment(var_decl.label, expr, parent_method));
                }
            }
        }
    }

    fn lower_expression(&mut self, e: &ast::Expression, parent_method: &ast::MethodDecl) -> ir::Expression {
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
                let left = self.lower_expression(&bin_op.left, parent_method);
                let right = self.lower_expression(&bin_op.right, parent_method);
                ir::Expression::Intrinsic(Box::new(
                    ir::Intrinsic::IntOp(bin_op.operator, left, right)))
            }
            ast::Expression::FieldAccess(ref fa) => {
                let target = self.lower_expression(&fa.target, parent_method);
                let field_label = self.query_engine.query_field(fa.label.assert_as_var_use());
                let field_id = self.fields[&field_label];
                ir::Expression::FieldAccess(Box::new(ir::FieldAccess { target, field_id }))
            }
            ast::Expression::Literal(ref l) => {
                ir::Expression::Literal(match &l.kind {
                    &ast::LiteralKind::Int(i) => ir::Literal::Int(i),
                    &ast::LiteralKind::String(ref s) => ir::Literal::String(s.clone()),
                    &ast::LiteralKind::Array(_, ref exprs) => {
                        let exprs = exprs.iter().map(|e| self.lower_expression(e, parent_method)).collect();
                        ir::Literal::Array(exprs)
                    },
                    &ast::LiteralKind::Null => ir::Literal::Null
                })
            }
            ast::Expression::MethodCall(ref mc) => {
                // FIXME: what if an intrinsic method is called? Console.WriteLine
                let label = self.query_engine.query_method_decl(mc.label.assert_as_method_use());
                let is_static = self.query_engine.query_is_static(label);
                let this = if is_static { None } else { Some(self.lower_expression(&mc.target, parent_method)) };
                self.lower_method_call(label, this, &mc.args, parent_method)
            }
            ast::Expression::New(ref n) => {
                let class_label = self.query_engine.query_class_decl(&n.class_name);
                ir::Expression::NewObject(class_label)
            }
            ast::Expression::Identifier(ref i) => {
                let var_label = self.query_engine.query_var_decl(i.label);
                ir::Expression::VarRead(self.var_tracker.get_var_id(var_label))
            }
            ast::Expression::This(ref t) => {
                if parent_method.is_static {
                    panic!("`this` keyword used inside static method");
                }

                // When used from a method, the first parameter will always be this
                ir::Expression::VarRead(VarId::this())
            }
        }
    }

    fn lower_assignment(&mut self, target: ast::Label, expr: &ast::Expression, parent_method: &ast::MethodDecl) -> ir::Statement {
        // Note: right now, the only lvalues are variables. No array indexing.
        // Type check
        let decl_label = self.query_engine.query_var_decl(target);
        let var_ty = self.query_engine.query_var_type(decl_label);
        let expr_ty = self.query_engine.query_expr_type(expr.label()).unwrap();
        if var_ty != expr_ty {
            panic!("Type mismatch in assignment: {:?} and {:?}", var_ty, expr_ty);
        }

        // Generate code
        let value = self.lower_expression(expr, parent_method);
        let var_id = self.var_tracker.get_var_id(decl_label);
        ir::Statement::Assign(ir::Assign { var_id, value })
    }

    fn lower_method_call(&mut self, method_label: labels::MethodDecl, this: Option<ir::Expression>, args: &[ast::Expression], parent_method: &ast::MethodDecl) -> ir::Expression {
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
            arguments.push(self.lower_expression(arg, parent_method));
        }

        ir::Expression::MethodCall(ir::MethodCall {
            method_id: self.methods[&method_label],
            arguments
        })
    }
}
