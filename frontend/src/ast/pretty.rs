use std::fmt::{self, Display};

use super::ast::*;

pub struct PrettyPrinter {
    indentation: u32
}

impl PrettyPrinter {
    pub fn new() -> PrettyPrinter {
        PrettyPrinter {
            indentation: 0
        }
    }

    // Utility methods
    fn bracket_open(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        self.indentation += 4;
        writeln!(f, "{{")
    }

    fn bracket_close(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        self.indentation -= 4;
        self.indent(f)?;
        writeln!(f, "}}")
    }

    fn indent(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.indentation {
            write!(f, " ")?;
        }

        Ok(())
    }

    fn comma_separated<T, F>(f: &mut fmt::Formatter, items: &[T], format: F) -> fmt::Result
    where
        F: Fn(&mut fmt::Formatter, &T) -> fmt::Result
    {
        if items.len() > 0 {
            let last = items.len() - 1;
            for x in &items[..last] {
                format(f, x)?;
                write!(f, ", ")?;
            }

            let x = &items[last];
            format(f, x)
        } else {
            Ok(())
        }
    }

    // AST-related
    pub fn print_program(&mut self, f: &mut fmt::Formatter, p: &Program) -> fmt::Result {
        for item in &p.items {
            self.print_top_item(f, item)?;
            writeln!(f)?;
        }

        Ok(())
    }

    pub fn print_top_item(&mut self, f: &mut fmt::Formatter, i: &TopItem) -> fmt::Result {
        let &TopItem::ClassDecl(ref cd) = i;
        write!(f, "class {}", cd.name)?;

        if let Some(ref superclass) = cd.superclass {
            write!(f, " : {} ", superclass)?;
        }

        self.bracket_open(f)?;

        for item in &cd.items {
            self.indent(f)?;
            // Note: all fields and methods are public in our implementation
            write!(f, "public ")?;
            match *item {
                ClassItem::FieldDecl(ref fd) => {
                    write!(f, "{} {}", fd.ty, fd.name)?;
                    if let Some(ref assignment) = fd.assignment {
                        self.print_expression(f, assignment)?;
                    }
                    writeln!(f, ";")?;
                }
                ClassItem::MethodDecl(ref md) => {
                    if md.is_static {
                        write!(f, "static ")?;
                    }

                    write!(f, "{} {}(", md.return_ty, md.name)?;
                    PrettyPrinter::comma_separated(f, &md.params, |f, param| {
                        write!(f, "{} {}", param.ty, param.name)
                    })?;
                    write!(f, ") ")?;
                    self.bracket_open(f)?;

                    for statement in &md.body {
                        self.print_statement(f, &statement)?;
                    }

                    self.bracket_close(f)?;
                }
            }
        }

        self.bracket_close(f)
    }

    pub fn print_statement(&mut self, f: &mut fmt::Formatter, s: &Statement) -> fmt::Result {
        self.indent(f)?;
        match *s {
            Statement::Assign(ref assign) => {
                write!(f, "{} = ", assign.var_name)?;
                self.print_expression(f, &assign.expr)?;
            }
            Statement::Expression(ref expr) => self.print_expression(f, expr)?,
            Statement::Return(ref ret) => {
                write!(f, "return")?;
                if let Some(ref expr) = ret.expr {
                    write!(f, " ")?;
                    self.print_expression(f, expr)?;
                }
            }
            Statement::VarDecl(ref decl) => {
                write!(f, "{} {}", decl.ty, decl.var_name)?;
                if let Some(ref expr) = decl.expr {
                    write!(f, " = ")?;
                    self.print_expression(f, expr)?;
                }
            }
        }

        writeln!(f, ";")
    }

    pub fn print_expression(&self, f: &mut fmt::Formatter, e: &Expression) -> fmt::Result {
        match *e {
            Expression::BinaryOp(ref op) => {
                self.print_expression(f, &op.left)?;
                write!(f, " {} ", op.operator)?;
                self.print_expression(f, &op.right)?;
            }
            Expression::FieldAccess(ref access) => {
                self.print_expression(f, &access.target)?;
                write!(f, ".{}", access.field_name)?;
            }
            Expression::Literal(_, ref l) => {
                l.fmt(f)?;
            }
            Expression::MethodCall(ref call) => {
                self.print_expression(f, &call.target)?;
                write!(f, ".{}(", call.method_name)?;
                PrettyPrinter::comma_separated(f, &call.args, |f, expr| self.print_expression(f, expr) )?;
                write!(f, ")")?;
            }
            Expression::New(ref new) => {
                write!(f, "new {}(", new.class_name)?;
                PrettyPrinter::comma_separated(f, &new.args, |f, expr| self.print_expression(f, expr) )?;
                write!(f, ")")?;
            }
            Expression::Identifier(ref s) => {
                s.name.fmt(f)?;
            }
            Expression::This(_) => {
                "this".fmt(f)?;
            }
        }

        Ok(())
    }
}