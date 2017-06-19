extern crate frontend;

mod interpreter;
mod ir;
mod lowering;

use frontend::analysis::QueryEngine;
use frontend::sample_programs;
use lowering::LoweringContext;

fn main() {
    let hw = sample_programs::hello_world();
    println!("=== Hello world:");
    println!("{}", hw);
    println!("=== Compiling");
    let mut query_engine = QueryEngine::new(&hw);
    let p = LoweringContext { query_engine: &mut query_engine }.lower_program();
    let entry_point = query_engine.query_entry_point();
    println!("=== Running");

    // FIXME: replace Vec::new by a real Vec<ClassInfo>
    interpreter::run(&p, entry_point, Vec::new());
}