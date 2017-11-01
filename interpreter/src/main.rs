extern crate frontend;
extern crate time;

mod interpreter;
mod ir;
mod lowering;

use frontend::analysis::QueryEngine;
//use frontend::ast::{ClassDecl, TopItem};
use frontend::sample_programs;
use lowering::LoweringContext;

fn main_() {
    let hw = sample_programs::large_fn();
    println!("=== Hello world:");
    println!("{}", hw);
    //println!("=== Compiling");
    let mut query_engine = QueryEngine::new(&hw);

    // {
    // println!("Query class decl for `Console`:");
    // let class_label = query_engine.query_class_decl("Console");
    // let class_decl: &ClassDecl = query_engine.nodes[&class_label.as_label()].downcast();
    // println!("{}", TopItem::ClassDecl(class_decl.clone()));
    // }

    //println!("Query entry point for the program: {}", query_engine.entry_point().name);

    //println!("=== After lowering");
    let output = LoweringContext::new(&hw, &mut query_engine).lower_program();
    //println!("{:?}", output.program);

    println!("=== Running");
    interpreter::run(&output.program, output.classes);
}

fn main() {
    test_get_ty();
    test_get_decl();
    test_get_available_methods();
}

fn test_get_ty() {
    use frontend::ast;
    let program = sample_programs::large_fn();

    // We need the node id of an expression that we want to know the type of
    // Since the large_fn program only has one method, and each statement in the method
    // is a var declaration with a corresponding assignment, we can take the last statement and
    // query the type of its expression
    let main = program.methods().next().unwrap();
    let last_statement = &main.body[main.body.len() - 1];
    let expr_label = match last_statement {
        &ast::Statement::VarDecl(ref vd) => vd.expr.as_ref().unwrap().label(),
        _ => unreachable!()
    };

    // Measure this
    let begin1 = time::precise_time_ns();
    let mut query_engine = QueryEngine::new(&program);
    let ty = query_engine.query_expr_type(expr_label).unwrap();
    let end1 = time::precise_time_ns();

    // Sanity check
    assert_eq!(ty, query_engine.types().int_ty());

    println!("Seconds: {}", (end1 - begin1) as f64 / 1000000000.0);

    // Now measure the performance of a compiler pass that type checks the whole program
}

fn test_get_decl() {
    use frontend::ast;
    let program = sample_programs::large_fn();

    // We need the node id of a variable use
    // Since the large_fn program only has one method, and each statement in the method
    // is a var declaration with an assignment to a binary operation between an operator, we can take the last statement and
    // query the type of its expression
    let main = program.methods().next().unwrap();
    let last_statement = &main.body[main.body.len() - 1];
    let var_use_label = match last_statement {
        &ast::Statement::VarDecl(ref vd) => {
            let expr = vd.expr.as_ref().unwrap();
            match expr {
                &ast::Expression::BinaryOp(ref e) => {
                    e.left.label()
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    };

    let var_decl_statement = &main.body[main.body.len() - 2];
    let var_decl_label = match var_decl_statement {
        &ast::Statement::VarDecl(ref vd) => vd.label,
        _ => unreachable!()
    };

    // Measure this
    let mut query_engine = QueryEngine::new(&program);
    let decl = query_engine.query_var_decl(var_use_label.as_label());
    assert_eq!(decl.as_label(), var_decl_label);

    // Now measure the performance of a compiler pass that does the same
}

fn test_get_available_methods() {
    use frontend::ast;
    let program = sample_programs::many_classes();

    // There are 1000 classes in this program, each with 3 methods

    // Measure this
    let mut query_engine = QueryEngine::new(&program);
    let decl = query_engine.query_class_decl("C955");
    let class = query_engine.classes()[&decl];
    assert_eq!(class.items.len(), 3);

    // Now measure the performance of a compiler pass that does the same
}

#[cfg(test)]
mod test {
    use interpreter;
    use ir::*;

    #[test]
    fn does_not_crash() {
        interpreter::run(&hello_world(), Default::default());
    }

    pub fn hello_world() -> Program {
        let methods = vec![
            Method {
                body: vec![
                    Statement::Expression(
                        Expression::Intrinsic(
                            Box::new(
                                Intrinsic::PrintLine(
                                    Expression::Literal(
                                        Literal::String("Hello world!".to_string())
                                    )
                                )
                            )
                        )
                    )
                ]
            }
        ];

        Program {
            methods,
            entry_point: MethodId(0)
        }
    }
}
