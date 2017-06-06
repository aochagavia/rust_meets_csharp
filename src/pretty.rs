use std::fmt::{self, Display};

use ast::*;

pub struct PrettyPrinter {
    pub indentation: u32
}

impl PrettyPrinter {
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
            write!(f, " : {}", superclass)?;
        }

        writeln!(f, "{{")?;

        for item in items {
            match item.content {
                ClassItem::FieldDecl { ref name, ref ty, ref assignment } => {
                    write!(f, "{} {}", ty, name)?;
                    if let &Some(ref assignment) = assignment {
                        self.print_expression(f, assignment)?;
                    }
                    writeln!(f, ";")?;
                }
                ClassItem::MethodDecl { ref name, ref params, ref body, ref return_ty } => {
                    write!(f, "{} {}(", return_ty, name)?;

                    if params.len() > 0 {
                        let last = params.len() - 1;
                        for &(ref param, ref ty) in &params[..last] {
                            write!(f, "{} {}, ", ty, param)?;
                        }

                        let (ref param, ref ty) = params[last];
                        write!(f, "{} {}", ty, param)?;
                    }

                    writeln!(f, ") {{")?;

                    for statement in body {
                        self.print_statement(f, &statement.content)?;
                    }

                    writeln!(f, "}}")?;
                }
            }
        }

        writeln!(f, "}}")
    }

    pub fn print_statement(&mut self, f: &mut fmt::Formatter, s: &Statement) -> fmt::Result {
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

    pub fn print_expression(&mut self, f: &mut fmt::Formatter, e: &Expression) -> fmt::Result {
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
                let last_param = params.len() - 1;
                if params.len() > 1 {
                    for expr in params {
                        self.print_expression(f, expr)?;
                        write!(f, ", ")?;
                    }

                    self.print_expression(f, &params[last_param])?;
                }
                write!(f, ")")?;
            }
            Expression::VarRead(ref s) => {
                s.fmt(f)?;
            }
        }

        Ok(())
    }
}