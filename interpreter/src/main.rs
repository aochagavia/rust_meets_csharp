extern crate frontend;

mod interpreter;
mod ir;
mod lowering;

use frontend::analysis::QueryEngine;
use frontend::sample_programs;
use lowering::LoweringContext;

fn main() {
    let hw = sample_programs::variables();
    println!("=== Program:");
    println!("{}", hw);

    // Compile
    let mut query_engine = QueryEngine::new(&hw);
    let output = LoweringContext::new(&hw, &mut query_engine).lower_program();

    // Run
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
