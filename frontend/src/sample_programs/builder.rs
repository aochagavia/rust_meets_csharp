use ast::*;

pub struct Builder {

}

impl Builder {
    pub fn decl_string(name: &str, assign: &str) -> Statement {
        let label = fresh_label();
        let var_name = name.to_string();
        let ty = Type::Custom("String".to_string());
        let expr = Expression::Literal(Literal { label: fresh_label(), kind: LiteralKind::String(assign.to_string()) });
        Statement::VarDecl(VarDecl { label, var_name, ty, expr: Some(expr) })
    }

    pub fn decl_int(name: &str, assign: i64) -> Statement {
        let label = fresh_label();
        let var_name = name.to_string();
        let ty = Type::Custom("int".to_string());
        let expr = Expression::Literal(Literal { label: fresh_label(), kind: LiteralKind::Int(assign) });
        Statement::VarDecl(VarDecl { label, var_name, ty, expr: Some(expr) })
    }

    pub fn decl_string_from_var(name: &str, var: &str) -> Statement {
        let label = fresh_label();
        let var_name = name.to_string();
        let ty = Type::Custom("String".to_string());
        let expr = Expression::Identifier(Identifier { label: fresh_label(), name: var.to_string() });
        Statement::VarDecl(VarDecl { label, var_name, ty, expr: Some(expr) })
    }

    pub fn decl_int_from_expr(name: &str, expr: Expression) -> Statement {
        let label = fresh_label();
        let var_name = name.to_string();
        let ty = Type::Custom("int".to_string());
        Statement::VarDecl(VarDecl { label, var_name, ty, expr: Some(expr) })
    }

    pub fn return_(expr: Expression) -> Statement {
        Statement::Return(Return {
            label: fresh_label(),
            expr: Some(expr)
        })
    }

    pub fn return_var(name: &str) -> Statement {
        let label = fresh_label();
        Statement::Return(Return {
            label,
            expr: Some(Builder::var_use(name))
        })
    }

    pub fn literal(lit: LiteralKind) -> Expression {
        Expression::Literal(Literal { label: fresh_label(), kind: lit })
    }

    pub fn var_use(name: &str) -> Expression {
        let label = fresh_label();
        let name = name.to_string();
        Expression::Identifier(Identifier { label, name })
    }

    pub fn binary_op(operator: BinaryOperator, left: Expression, right: Expression) -> Expression {
        Expression::BinaryOp(BinaryOp {
            label: fresh_label(),
            operator,
            left: Box::new(left),
            right: Box::new(right)
        })
    }

    pub fn sum_vars(x: &str, y: &str) -> Expression {
        let label = fresh_label();
        let operator = BinaryOperator::Add;
        let left = Box::new(Builder::var_use(x));
        let right = Box::new(Builder::var_use(y));
        Expression::BinaryOp(BinaryOp { label, operator, left, right })
    }

    pub fn write_line(var: &str) -> Statement {
        Statement::Expression(Builder::method_call("Console", "WriteLine", vec![var]))
    }

    pub fn write_line_str(msg: &str) -> Statement {
        Statement::Expression(Builder::method_call_literal("Console", "WriteLine", vec![LiteralKind::String(msg.to_string())]))
    }

    pub fn write_line_expr(expr: Expression) -> Statement {
        Statement::Expression(Builder::method_call_expr("Console", "WriteLine", vec![expr]))
    }

    pub fn if_then_else(condition: Expression, then: Vec<Statement>, else_: Vec<Statement>) -> Statement {
        Statement::IfThenElse(IfThenElse {
            label: fresh_label(),
            condition,
            then,
            else_
        })
    }

    pub fn method_call(class: &str, method: &str, vars: Vec<&str>) -> Expression {
        Builder::method_call_expr(class, method, vars.into_iter().map(Builder::var_use).collect())
    }

    pub fn method_call_literal(class: &str, method: &str, literals: Vec<LiteralKind>) -> Expression {
        Builder::method_call_expr(class, method, literals.into_iter().map(Builder::literal).collect())
    }

    pub fn method_call_expr(class: &str, method: &str, args: Vec<Expression>) -> Expression {
        Expression::MethodCall(MethodCall {
            label: fresh_label(),
            target: Box::new(Expression::Identifier(Identifier { name: class.to_string(), label: fresh_label() })),
            method_name: method.to_string(),
            args
        })
    }
}
