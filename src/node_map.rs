use std::collections::HashMap;

use ast;

pub enum Node {
    TopItem(ast::TopItem),
    ClassItem(ast::ClassItem),
    Statement(ast::Statement),
}

// A map from node labels to nodes
pub struct NodeMap {
    map: HashMap<ast::Label, Node>
}

impl NodeMap {
    pub fn build(p: &ast::Program) -> NodeMap {
        NodeMap {
            map: HashMap::new()
        }
    }

    pub fn get_node(&self, label: ast::Label) -> Option<Node> {
        None
    }
}