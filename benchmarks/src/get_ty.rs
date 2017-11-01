use time;

use frontend::analysis::{self, QueryEngine};
use frontend::ast;
use frontend::sample_programs;
use type_checker;

pub fn prepare() -> (ast::Program, analysis::labels::Expression) {
    let program = sample_programs::large_fn();

    // We need the node id of an expression that we want to know the type of
    // Since the large_fn program only has one method, and each statement in the method
    // is a var declaration with a corresponding assignment, we can take the last statement and
    // query the type of its expression

    let expr_label = {
        let main = program.methods().next().unwrap();
        let last_statement = &main.body[main.body.len() - 1];
        match last_statement {
            &ast::Statement::VarDecl(ref vd) => vd.expr.as_ref().unwrap().label(),
            _ => unreachable!()
        }
    };

    (program, expr_label)
}

pub fn on_demand(program: &ast::Program, expr_label: analysis::labels::Expression) -> u64 {
    let begin = time::precise_time_ns();
    let mut query_engine = QueryEngine::new(&program);
    let ty = query_engine.query_expr_type(expr_label).unwrap();
    let end = time::precise_time_ns();

    // Sanity check
    assert_eq!(ty, query_engine.types().int_ty());

    end - begin
}

pub fn traditional(program: &ast::Program, expr_label: ast::Label) -> u64 {
    let begin = time::precise_time_ns();
    let (types, type_map) = type_checker::check(&program);
    let ty = types[&expr_label];
    let end = time::precise_time_ns();

    // Sanity check
    assert_eq!(ty, type_map.int_ty());

    end - begin
}
