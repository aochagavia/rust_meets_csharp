use time;

use frontend::analysis::{labels, AstPreprocessor, QueryEngine};
use frontend::ast;
use frontend::sample_programs;

pub fn prepare() -> (ast::Program, labels::VarDecl, labels::VarUse) {
    let program = sample_programs::large_fn();

    // We need the node id of a variable use
    // Since the large_fn program only has one method, and each statement in the method
    // is a var declaration with an assignment to a binary operation between an operator, we can take the last statement and
    // query the type of its expression
    let (var_decl_label, var_use_label) = {
        let main = program.methods().next().unwrap();
        let last_statement = &main.body[main.body.len() - 1];
        let var_use_label = match last_statement {
            &ast::Statement::VarDecl(ref vd) => {
                let expr = vd.expr.as_ref().unwrap();
                match expr {
                    &ast::Expression::BinaryOp(ref e) => {
                        e.left.label().as_label().assert_as_var_use()
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        };

        let var_decl_statement = &main.body[main.body.len() - 2];
        let var_decl_label = match var_decl_statement {
            &ast::Statement::VarDecl(ref vd) => vd.label.assert_as_var_decl(),
            _ => unreachable!()
        };

        (var_decl_label, var_use_label)
    };

    (program, var_decl_label, var_use_label)
}

pub fn on_demand(program: &ast::Program, var_decl_label: labels::VarDecl, var_use_label: labels::VarUse) -> u64 {
    let begin = time::precise_time_ns();
    let mut query_engine = QueryEngine::new(&program);
    let decl = query_engine.query_var_decl(var_use_label.as_label());
    let end = time::precise_time_ns();

    // Sanity check
    assert_eq!(decl.as_label(), var_decl_label.as_label());

    end - begin
}

pub fn traditional(program: &ast::Program, var_decl_label: labels::VarDecl, var_use_label: labels::VarUse) -> u64 {
    let begin = time::precise_time_ns();
    let data = AstPreprocessor::preprocess(program);
    let decl = data.var_map[&var_use_label.as_label()];
    let end = time::precise_time_ns();

    // Sanity check
    assert_eq!(decl.label, var_decl_label.as_label());

    end - begin
}
