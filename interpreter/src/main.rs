extern crate frontend;

mod interpreter;
mod ir;
mod lowering;

use frontend::analysis::QueryEngine;
//use frontend::ast::{ClassDecl, TopItem};
use frontend::sample_programs;
use lowering::LoweringContext;

fn main() {
    let hw = sample_programs::hello_world();
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

    let output = LoweringContext::new(&hw, &mut query_engine).lower_program();
    println!("=== Running");

    interpreter::run(&output.program, output.classes);
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
