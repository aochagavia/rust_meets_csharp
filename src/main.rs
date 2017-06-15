mod ast;
mod identifier_collector;
mod maps;
mod pretty;
mod programs;
mod visitor;

use identifier_collector::IdentifierCollector;
use maps::{DefMap, Node, NodeMap};
use visitor::Visitor;

fn main() {
    let hw = programs::hello_world();
    println!("=== Hello world:");
    println!("{}", hw);
    let ctx = Context::new(&hw);
    let mut ic = IdentifierCollector::new();
    ic.visit_program(&hw);
    println!("=== Identifiers:");
    for (ident, labels) in ic.idents {
        println!("{} => {:?}", ident, labels);
    }
    //println!("Nodemap: {:?}", nm);
    //let l = find_label_for_elem(&hw, "Console").expect("Label not found");
    //println!("Method list for Console: {:?}", get_method_list(&hw, &nm, l).unwrap());
}

struct Context<'a> {
    pub program: &'a ast::Program,
    pub nodes: NodeMap<'a>,
    pub definitions: DefMap<'a>
}

impl<'a> Context<'a> {
    fn new(program: &'a ast::Program) -> Context {
        let (nodes, definitions) = maps::build(program);
        Context { program, nodes, definitions }
    }

    fn get_method_list(&'a mut self, label: ast::Label) -> Option<Vec<&'a str>> {
        // Is the target of the method a class name or a variable name?
        match self.get_class_definition(label) {
            Some(_) => {
                // We want to retrieve all static methods available for this class
            }
            None => {

            }
        }
        // If class name, get static method list
        // If variable name, get type of the variable and query available non-static methods

        // Now what?
        Some(vec!["bananas"])
    }

    fn get_class_definition(&'a mut self, label: ast::Label) -> Option<ast::Label> {
        // Only proceed if the label points to an identifier
        let ident = match self.nodes.get(label) {
            Some(Node::Identifier(i)) => i,
            _ => return None
        };

        // Look up the class definition associated to this name
        self.definitions.get(&ident.name)
    }

    fn get_static_methods(&'a mut self, label: ast::Label) -> Vec<&'a str> {
        let class = match self.nodes.get(label) {
            Some(Node::ClassDecl(c)) => c,
            _ => panic!("Called get_static_methods on a node that is not a class definition")
        };

        panic!()
    }
}
