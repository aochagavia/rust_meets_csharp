//! This module exposes an interpreter for the subset of C# that we suppport
//!
//! It is intended as a proof of concept to show that the on-demand nature
//! of our analysis also works well in a compiler setting.

mod interpreter;
mod runtime;

use frontend::analysis::{self, MethodId};

pub fn run(program: &::ir::Program, entry_point: MethodId, classes: Vec<analysis::ClassInfo>) {
    self::interpreter::Interpreter {
        classes,
        program,
        stack: Vec::new(),
        stack_ptr: 0
    }.run(entry_point);
}
