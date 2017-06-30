use std::collections::HashMap;

use frontend::analysis::labels;
use super::VarId;

#[derive(Default)]
pub struct VarTracker {
    vars: HashMap<labels::VarDecl, VarId>
}

impl VarTracker {
    pub fn reset(&mut self) {
        self.vars.clear()
    }

    pub fn var_decl(&mut self, var_decl: labels::VarDecl) {
        let var_id = VarId(self.vars.len());
        self.vars.insert(var_decl, var_id);
    }

    pub fn get_var_id(&mut self, var_decl: labels::VarDecl) -> VarId {
        // FIXME: this will crash if the variable is undefined
        self.vars[&var_decl]
    }
}