//! This module exposes an interpreter for the subset of C# that we suppport
//!
//! It is intended as a proof of concept to show that the on-demand nature
//! of our analysis also works well in a compiler setting.

mod interpreter;
mod runtime;

pub fn run(p: &::ir::Program, ctx: ::lowering::ProgramMetadata) {
    self::interpreter::Interpreter {
        classes: ctx.classes,
        methods: ctx.methods,
        stack: Vec::new(),
        stack_ptr: 0
    }.run(p);
}
