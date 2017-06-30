//! This module exposes an interpreter for the subset of C# that we suppport
//!
//! It is intended as a proof of concept to show that the on-demand nature
//! of our analysis also works well in a compiler setting.

mod interpreter;
mod runtime;

use std::collections::HashMap;

use frontend::analysis::labels;
use lowering::ClassInfo;

pub fn run(program: &::ir::Program, classes: HashMap<labels::ClassDecl, ClassInfo>) {
    self::interpreter::Interpreter {
        classes,
        program,
        stack: Vec::new(),
        stack_ptr: 0
    }.run();
}
