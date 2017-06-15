//! This module exposes an interpreter for the subset of C# that we suppport
//!
//! It is intended as a proof of concept to show that the on-demand nature
//! of our analysis also works well in a compiler setting.

mod ir;
mod runtime;

use self::runtime as rt;

// FIXME: this information is statically known... Right now, we leave it as-is,
// because it makes lowering easier. In the future it would make sense to let the
// IR be a graph, similar to Rust's MIR. We could split the code into basic blocks,
// where the statements are executed in sequential order and at the end jump to another
// basic block. This will become especially interesting when we introduce control flow
// stuff such as if-then-else statements and loops.
enum Action {
    NextStatement,
    Return(Option<rt::Value>)
}

struct ClassInfo {
    superclass_id: Option<usize>,
    name: String,
    field_names: Vec<String>,
}

pub struct Interpreter {
    classes: Vec<ClassInfo>,
    methods: Vec<ir::Method>,
    stack: Vec<rt::Value>,
    stack_ptr: usize,
}

impl Interpreter {
    pub fn run(&mut self, p: &ir::Program) {
        self.run_method(&p.entry_point, vec![]);
    }

    // Note: var_id is 0-based
    fn stack_addr(&self, var_id: usize) -> usize {
        self.stack_ptr + var_id
    }

    fn run_method(&mut self, m: &ir::Method, args: Vec<rt::Value>) -> Option<rt::Value> {
        // Allocate arguments on the stack
        let sp = self.stack_ptr;
        self.stack_ptr = self.stack.len();
        self.stack.extend(args);

        // Run the statements
        let mut ret = None;
        for s in &m.body {
            if let Action::Return(val) = self.run_statement(s) {
                ret = val;
                break;
            }
        }

        // Free stack space
        self.stack_ptr = sp;
        self.stack.truncate(sp);

        ret
    }

    fn run_statement(&mut self, s: &ir::Statement) -> Action {
        use self::ir::Statement::*;
        match *s {
            Assign(ref assign) => {
                let addr = self.stack_addr(assign.var_id);
                self.stack[addr] = self.run_expression(&assign.value);
                Action::NextStatement
            }
            Expression(ref expr) => {
                self.run_expression(expr);
                Action::NextStatement
            }
            Return(ref val) => {
                Action::Return(val.as_ref().map(|v| self.run_expression(v)))
            }
            VarDecl => {
                self.stack.push(rt::Value::Int(::std::i64::MAX));
                Action::NextStatement
            }
        }
    }

    fn run_expression(&mut self, e: &ir::Expression) -> rt::Value {
        use self::ir::Expression::*;
        match *e {
            FieldAccess(ref fa) => {
                let addr = self.stack_addr(fa.var_id);
                match self.stack[addr] {
                    // FIXME: we do nothing to deal with by ref vs by val
                    rt::Value::Object(ref obj) => obj.fields[fa.field_id].clone(),
                    _ => unreachable!()
                }
            }
            Literal(ref l) => {
                self.run_literal(l)
            }
            Intrinsic(ref i) => {
                self.run_intrinsic(i)
            }
            MethodCall(ref mc) => {
                // Obtain method based on method_id
                // FIXME: do something better than cloning
                let method = &self.methods[mc.method_id].clone();
                let args = mc.arguments.iter().map(|expr| self.run_expression(expr)).collect();

                // Note: for void methods we just return null. The return type will be ignored anyway.
                self.run_method(method, args).unwrap_or(rt::Value::Null)
            }
            VarRead(var_id) => {
                let addr = self.stack_addr(var_id);
                // FIXME: we do nothing to deal with by ref vs by val
                self.stack[addr].clone()
            }
            NewObject(class_id) => {
                let fields = vec![rt::Value::Null; self.classes[class_id].field_names.len()];
                rt::Value::Object(rt::Object { class_id, fields })
            }
        }
    }

    fn run_literal(&mut self, l: &ir::Literal) -> rt::Value {
        use self::ir::Literal::*;
        match *l {
            Int(i) => rt::Value::Int(i),
            String(ref s) => rt::Value::String(s.clone()),
            Array(ref exprs) => rt::Value::Array(exprs.iter().map(|e| self.run_expression(e)).collect()),
            Null => rt::Value::Null
        }
    }

    fn run_intrinsic(&mut self, i: &ir::Intrinsic) -> rt::Value {
        use self::ir::Intrinsic::*;
        use ast::BinaryOperator::*;
        match *i {
            IntOp(ref op, ref e1, ref e2) => {
                let e1 = self.run_expression(e1);
                let e2 = self.run_expression(e2);
                let (e1, e2) = match (e1, e2) {
                    (rt::Value::Int(e1), rt::Value::Int(e2)) => (e1, e2),
                    _ => unreachable!()
                };
                let res = match *op {
                    Add => e1 + e2,
                    Sub => e1 - e2,
                    Mul => e1 * e2,
                    Div => e1 / e2,
                };
                rt::Value::Int(res)
            }
            PrintLine(ref expr) => {
                let val = self.run_expression(expr);
                self.print_value(&val);
                println!();

                // Return null. The type system ensures this return value will be ignored anyway
                rt::Value::Null
            }
        }
    }

    fn print_value(&self, v: &rt::Value) {
        match *v {
            rt::Value::String(ref s) => print!("{}", s),
            rt::Value::Array(ref v) => {
                print!("[");
                if v.len() > 0 {
                    let last = v.len() - 1;
                    for x in &v[..last] {
                        self.print_value(x);
                        print!(", ");
                    }

                    let x = &v[last];
                    self.print_value(x);
                }
                print!("]");
            }
            rt::Value::Int(i) => print!("{}", i),
            rt::Value::Object(ref obj) => {
                let class = &self.classes[obj.class_id];
                println!("{} {{", class.name);
                // Note: superclass fields are included in this list
                for (name, value) in class.field_names.iter().zip(obj.fields.iter()) {
                    print!("    {}: ", name);
                    self.print_value(value);
                    println!(",");
                }
                print!("}}");
            }
            rt::Value::Null => print!("null")
        }
    }
}
