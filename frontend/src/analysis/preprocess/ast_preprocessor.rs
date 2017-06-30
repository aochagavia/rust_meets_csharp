use std::collections::HashMap;

use analysis::labels;
use ast::*;

// How would this work when updating a file?

// FIXME: remove Default trait implementation
#[derive(Default)]
pub struct AstPreprocessor {
    pub node_map: HashMap<Label, Node>,
    pub parent_map: HashMap<Label, Label>,
    pub entry_points: Vec<labels::MethodDecl>
}

impl AstPreprocessor {
    pub fn new(p: &Program) -> AstPreprocessor {
        let mut basic = AstPreprocessor::default();

        for (path, file) in &p.files {
            basic.update_file(path.to_string(), file);
        }

        basic
    }

    pub fn remove_file(&mut self, path: String) {

    }

    pub fn update_file(&mut self, path: String, file: &File) {
        // Build a map from labels to nodes
        // Build a map from nodes to their parents
        // Look for entry points
    }

    pub fn entry_point(&mut self) -> labels::MethodDecl {
        // The entry point is a function with the following properties:
        // * Is static
        // * Is called Main
        if self.entry_points.len() != 1 {
            println!("Expected 1 entry point, found {}", self.entry_points.len());
        }

        self.entry_points[0]
    }
}
