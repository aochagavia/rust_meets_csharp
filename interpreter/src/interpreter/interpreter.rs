use std::collections::HashMap;

use frontend::analysis::labels;
use lowering::ClassInfo;
use ir;
use super::runtime as rt;

enum NextAction {
    Continue,
    Return(Option<rt::Value>)
}

pub struct Interpreter<'a> {
    pub classes: HashMap<labels::ClassDecl, ClassInfo>,
    pub stack: Vec<rt::Value>,
    pub stack_ptr: usize,
    pub program: &'a ir::Program
}

impl<'a> Interpreter<'a> {
    pub fn run(&mut self) {
        // FIXME: remove clone
        let ep = self.program.entry_point.0;
        let method = self.program.methods[ep].clone();
        self.run_method(&method, vec![]);
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
            if let NextAction::Return(val) = self.run_statement(s) {
                ret = val;
                break;
            }
        }

        // Free stack space
        self.stack_ptr = sp;
        self.stack.truncate(sp);

        ret
    }

    fn run_statement(&mut self, s: &ir::Statement) -> NextAction {
        use self::ir::Statement::*;
        match *s {
            Assign(ref assign) => {
                let addr = self.stack_addr(assign.var_id.0);
                self.stack[addr] = self.run_expression(&assign.value);
                NextAction::Continue
            }
            Expression(ref expr) => {
                self.run_expression(expr);
                NextAction::Continue
            }
            Return(ref val) => {
                NextAction::Return(val.as_ref().map(|v| self.run_expression(v)))
            }
            VarDecl => {
                self.stack.push(rt::Value::Int(::std::i64::MAX));
                NextAction::Continue
            }
        }
    }

    fn run_expression(&mut self, e: &ir::Expression) -> rt::Value {
        use self::ir::Expression::*;
        match *e {
            FieldAccess(ref fa) => {
                let target = self.run_expression(&fa.target);
                // Because of type checking, we know this is an object
                match target {
                    // FIXME: we do nothing to deal with by ref vs by val
                    rt::Value::Object(ref obj) => obj.fields[fa.field_id.0].clone(),
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
                let method = &self.program.methods[mc.method_id.0].clone();
                let args = mc.arguments.iter().map(|expr| self.run_expression(expr)).collect();

                // Note: for void methods we just return null. The return type will be ignored anyway.
                self.run_method(method, args).unwrap_or(rt::Value::Null)
            }
            VarRead(var_id) => {
                let addr = self.stack_addr(var_id.0);
                // FIXME: we do nothing to deal with by ref vs by val
                self.stack[addr].clone()
            }
            NewObject(class) => {
                let fields = vec![rt::Value::Null; self.classes[&class].field_names.len()];
                rt::Value::Object(rt::Object { class, fields })
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
        use frontend::ast::BinaryOperator::*;
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
                let class = &self.classes[&obj.class];
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
