use ast::*;
use super::Builder;

fn statements() -> Vec<Statement> {
    let amount = 100_000;
    let mut statements = Vec::with_capacity(amount);
    for var_count in 0..amount {
        let var_name = format!("a{}", var_count);
        let prev_expr = if var_count == 0 {
            Builder::literal(LiteralKind::Int(42))
        } else {
            let prev_var = format!("a{}", var_count - 1);
            Builder::binary_op(BinaryOperator::Add, Builder::var_use(&prev_var), Builder::literal(LiteralKind::Int(1)))
        };
        statements.push(Builder::decl_int_from_expr(&var_name, prev_expr))
    }
    statements
}

fn program_class() -> TopItem {
    let main_method = ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: "Main".to_string(),
        params: Vec::new(),
        body: statements(),
        is_static: true,
        return_ty: Type::Void
    });

    TopItem::ClassDecl(ClassDecl {
        label: fresh_label(),
        name: "Program".to_string(),
        items: vec![main_method]
    })
}

pub fn large_fn() -> Program {
    Program {
        items: vec![
            program_class()
        ]
    }
}
