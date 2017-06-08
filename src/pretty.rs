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
            self.print_top_item(f, &item.content)?;
            writeln!(f)?;
        }

        Ok(())
    }

    pub fn print_top_item(&mut self, f: &mut fmt::Formatter, i: &TopItem) -> fmt::Result {
        let &TopItem::ClassDecl { ref name, ref inherits_from, ref items } = i;
        write!(f, "class {}", name)?;

        if let &Some(ref superclass) = inherits_from {
            write!(f, " : {} ", superclass)?;
        }

        self.bracket_open(f)?;

        for item in items {
            match item.content {
                ClassItem::FieldDecl { ref name, ref ty, ref assignment } => {
                    self.indent(f)?;
                    write!(f, "{} {}", ty, name)?;
                    if let &Some(ref assignment) = assignment {
                        self.print_expression(f, assignment)?;
                    }
                    writeln!(f, ";")?;
                }
                ClassItem::MethodDecl { ref name, ref params, ref body, ref return_ty } => {
                    self.indent(f)?;
                    write!(f, "{} {}(", return_ty, name)?;
                    PrettyPrinter::param_list(f, params, |f, p| {
                        let &(ref param, ref ty) = p;
                        write!(f, "{} {}", ty, param)
                    })?;
                    write!(f, ") ")?;
                    self.bracket_open(f)?;

                    for statement in body {
                        self.print_statement(f, &statement.content)?;
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
            Statement::Assign { ref var_name, ref expr } => {
                write!(f, "{} = ", var_name)?;
                self.print_expression(f, expr)?;
            }
            Statement::Expression(ref expr) => self.print_expression(f, expr)?,
            Statement::Return(ref expr) => {
                write!(f, "return")?;
                if let &Some(ref expr) = expr {
                    write!(f, " ")?;
                    self.print_expression(f, expr)?;
                }
            }
            Statement::VarDecl { ref var_name, ref ty, ref expr } => {
                write!(f, "{} {}", ty, var_name)?;
                if let &Some(ref expr) = expr {
                    write!(f, " = ")?;
                    self.print_expression(f, expr)?;
                }
            }
        }

        writeln!(f, ";")
    }

    pub fn print_expression(&self, f: &mut fmt::Formatter, e: &Expression) -> fmt::Result {
        match *e {
            Expression::BinaryOp { ref operator, ref left, ref right } => {
                self.print_expression(f, &left)?;
                write!(f, " {} ", operator)?;
                self.print_expression(f, &right)?;
            }
            Expression::FieldAccess { ref variable, ref field_name } => {
                write!(f, "{}.{}", variable, field_name)?;
            }
            Expression::Literal(ref l) => {
                l.fmt(f)?;
            }
            Expression::MethodCall { ref target, ref method_name, ref params } => {
                write!(f, "{}.{}(", target, method_name)?;
                PrettyPrinter::param_list(f, params, |f, expr| self.print_expression(f, expr) )?;
                write!(f, ")")?;
            }
            Expression::New { ref class_name, ref params } => {
                write!(f, "new {}(", class_name)?;
                PrettyPrinter::param_list(f, params, |f, expr| self.print_expression(f, expr) )?;
                write!(f, ")")?;
            }
            Expression::VarRead(ref s) => {
                s.fmt(f)?;
            }
        }

        Ok(())
    }
}