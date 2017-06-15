use std::collections::HashMap;

use ast::{self, Identifier};
use ast::visitor::{self, Visitor};

pub struct IdentifierCollector<'a> {
    pub idents: HashMap<&'a str, Vec<ast::Label>>
}

impl<'a> IdentifierCollector<'a> {
    pub fn new() -> IdentifierCollector<'a> {
        IdentifierCollector {
            idents: HashMap::new()
        }
    }

    fn get_ident_labels<'b>(&'b mut self, ident: &str) -> &'b [ast::Label] {
        &self.idents[ident]
    }
}

impl<'a> Visitor<'a> for IdentifierCollector<'a> {
    fn visit_identifier(&mut self, ident: &'a Identifier) {
        self.idents.entry(&ident.name).or_insert(Vec::new()).push(ident.label);
        visitor::walk_identifier(self, ident)
    }
}
