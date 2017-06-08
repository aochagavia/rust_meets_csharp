use std::fmt::{self, Display};

use ast::*;

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

    fn param_list<T, F>(f: &mut fmt::Formatter, params: &[T], format: F) -> fmt::Result
    where
        F: Fn(&mut fmt::Formatter, &T) -> fmt::Result
    {
        if params.len() > 0 {
            let last = params.len() - 1;
            for x in &params[..last] {
                format(f, x)?;
                write!(f, ", ")?;
            }

            let x = &params[last];
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
            match *item {
                ClassItem::FieldDecl(ref fd) => {
                    self.indent(f)?;
                    write!(f, "{} {}", fd.ty, fd.name)?;
                    if let Some(ref assignment) = fd.assignment {
                        self.print_expression(f, assignment)?;
                    }
                    writeln!(f, ";")?;
                }
                ClassItem::MethodDecl(ref md) => {
                    self.indent(f)?;
                    write!(f, "{} {}(", md.return_ty, md.name)?;
                    PrettyPrinter::param_list(f, &md.params, |f, p| {
                        let &(ref param, ref ty) = p;
                        write!(f, "{} {}", ty, param)
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
                write!(f, "{}.{}", access.var_name, access.field_name)?;
            }
            Expression::Literal(ref l) => {
                l.fmt(f)?;
            }
            Expression::MethodCall(ref call) => {
                write!(f, "{}.{}(", call.target, call.method_name)?;
                PrettyPrinter::param_list(f, &call.params, |f, expr| self.print_expression(f, expr) )?;
                write!(f, ")")?;
            }
            Expression::New(ref new) => {
                write!(f, "new {}(", new.class_name)?;
                PrettyPrinter::param_list(f, &new.params, |f, expr| self.print_expression(f, expr) )?;
                write!(f, ")")?;
            }
            Expression::VarRead(ref s) => {
                s.fmt(f)?;
            }
        }

        Ok(())
    }
}