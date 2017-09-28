use std::collections::HashMap;

//use analysis::labels;
use ast::*;
use ast::visitor::Visitor;

use super::visitor::PreprocessVisitor;

pub struct AstData<'a> {
    pub nodes: HashMap<Label, Node<'a>>,
    pub classes_by_name: HashMap<&'a str, &'a ClassDecl>,
    pub entry_point: &'a MethodDecl
}

pub struct AstPreprocessor;
impl AstPreprocessor {
    pub fn preprocess(p: &Program) -> AstData {
        let mut visitor = PreprocessVisitor::default();
        visitor.visit_ast(&p.items);

        if visitor.errors.len() > 0 {
            println!("Errors while preprocessing:");
            for err in &visitor.errors {
                println!("{:?}", err);
            }

            panic!()
        }

        let ep = match visitor.entry_point {
            Some(e) => e,
            None => {
                panic!("No entry point found")
            }
        };

        AstData {
            nodes: visitor.nodes,
            classes_by_name: visitor.classes_by_name,
            entry_point: ep
        }
    }
}
