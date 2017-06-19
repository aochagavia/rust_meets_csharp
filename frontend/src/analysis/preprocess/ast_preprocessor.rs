use std::collections::HashMap;

use analysis::{ClassId, MethodId, Node};
use ast::*;

// How would this work when updating a file?
// Problem: MethodId, ClassId, VarId, FieldId are tightly coupled to the interpreter...
// The interpreter assumes the ids are continuous!
// We need to decouple them before it becomes possible to go further

// FIXME: remove Default trait implementation
#[derive(Default)]
pub struct AstPreprocessor {
    pub label_map: HashMap<Label, Node>,
    pub parent_map: HashMap<Label, Label>,
    pub method_map: HashMap<MethodId, Label>,
    pub class_map: HashMap<ClassId, Label>,
    pub entry_point: Option<MethodId>
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
        // Also used when adding a file
    }

    pub fn methods(&self) -> Vec<MethodDecl> {
        unimplemented!()
    }

    pub fn classes(&self) -> Vec<ClassDecl> {
        unimplemented!()
    }

    pub fn query_entry_point(&mut self) -> MethodId {
        // Go through all methods until we find one that has the following properties:
        // * Is static
        // * Is called Main

        // FIXME: what to do about multiple entry points? Should we query all methods?
        unimplemented!()
    }
}
