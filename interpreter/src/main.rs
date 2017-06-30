extern crate frontend;

mod interpreter;
mod ir;
mod lowering;

use frontend::analysis::QueryEngine;
use frontend::sample_programs;
use lowering::LoweringContext;

fn main() {
    let mut hw = sample_programs::hello_world();
    println!("=== Hello world:");
    println!("{}", hw);
    println!("=== Compiling");
    let mut query_engine = QueryEngine::new(&hw);
    let output = LoweringContext::new(&hw, &mut query_engine).lower_program();
    println!("=== Running");

    interpreter::run(&output.program, output.classes);
}
