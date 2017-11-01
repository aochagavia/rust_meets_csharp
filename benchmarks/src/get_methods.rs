use time;

use frontend::{ast, sample_programs};
use frontend::analysis::{AstPreprocessor, QueryEngine};

pub fn prepare() -> ast::Program {
    sample_programs::many_classes()
}

pub fn on_demand(program: &ast::Program) -> u64 {
    let begin = time::precise_time_ns();
    let mut query_engine = QueryEngine::new(&program);
    let decl = query_engine.query_class_decl("C955");
    let class = query_engine.nodes[&decl.as_label()].downcast::<ast::ClassDecl>();
    let end = time::precise_time_ns();

    // Sanity check
    assert_eq!(class.items.len(), 3);

    end - begin
}

pub fn traditional(program: &ast::Program) -> u64 {
    let begin = time::precise_time_ns();
    let data = AstPreprocessor::preprocess(&program);
    let methods = &data.classes_by_name["C955"].items;
    let end = time::precise_time_ns();

    // Sanity check
    assert_eq!(methods.len(), 3);

    end - begin
}
