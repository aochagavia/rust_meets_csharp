#![allow(dead_code, unused_variables)]

mod ast;
mod identifier_collector;
mod interpreter;
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
    let mut ctx = Context::new(&hw);
    let mut ic = IdentifierCollector::new();
    ic.visit_program(&hw);
    println!("=== Identifiers:");
    for (ident, labels) in ic.idents {
        println!("{} => {:?}", ident, labels);
    }
    println!("=== Available methods for Console:");
    for s in ctx.get_method_list(9).expect("Get method list failed") {
        println!("{}", s);
    }
}

struct Context<'a> {
    pub program: &'a ast::Program,
    pub nodes: NodeMap<'a>,
    pub definitions: DefMap<'a>,
}

impl<'a> Context<'a> {
    fn new(program: &'a ast::Program) -> Context<'a> {
        let (nodes, definitions) = maps::build(program);
        Context {
            program: program,
            nodes: nodes,
            definitions: definitions,
        }
    }

    fn get_method_list<'b>(&'b mut self, label: ast::Label) -> Option<Vec<&'b str>> {
        match self.get_class_definition(label) {
            Some(def_label) => {
                // If we are here, it means that the target is an identifier and that
                // it corresponds to a class definition.
                // FIXME: we are assuming that class names cannot be shadowed!
                // Is name resolution on-demand in Roslyn?
                Some(self.get_static_methods(def_label))
            }
            None => {
                // If we are here, it means that the target of the call is an expression
                // Therefore, we should somehow jump to its type definition and go further from there
                None
            }
        }
    }

    fn get_class_definition<'b>(&'b mut self, label: ast::Label) -> Option<ast::Label> {
        // Only proceed if the label points to an identifier
        let ident = match self.nodes.get(label) {
            Some(Node::Identifier(i)) => i,
            _ => return None,
        };

        // Look up the class definition associated to this name
        self.definitions.get(&ident.name)
    }

    fn get_static_methods<'b>(&'b mut self, label: ast::Label) -> Vec<&'b str> {
        let class = match self.nodes.get(label) {
            Some(Node::ClassDecl(c)) => c,
            _ => panic!("Called get_static_methods on a node that is not a class definition"),
        };

        // First, let's see whether this information has already been cached
        // FIXME: implement memoizing... could we do that automatically?

        // Otherwise, proceed!
        let mut ret = Vec::new();
        ret.extend(class.items.iter().filter_map(|i| i.method_decl()).map(|d| &*d.name));

        // Don't forget the superclass
        if let Some(ref superclass) = class.superclass {
            // FIXME: we are assuming a correct program here...
            let def = self.definitions.get(superclass).unwrap();
            ret.extend(self.get_static_methods(def));
        }

        ret
    }
}
