use ast::*;
use super::Builder;

fn program_class() -> TopItem {
    let main_method = ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: "Main".to_string(),
        params: Vec::new(),
        body: vec![
            Builder::write_line_str("Part one"),
            Builder::write_line_str("========"),
            Builder::decl_string("msg", "Hello there!"),
            Builder::decl_string_from_var("msg_copy", "msg"),
            Builder::write_line("msg_copy"),
            Builder::decl_int("x", 42),
            Builder::decl_int_from_expr("y", Builder::method_call("Program", "Aux", vec!["x"])),
            Builder::decl_int_from_expr("z", Builder::method_call("Program", "Aux", vec!["y"])),
            Builder::write_line("x"),
            Builder::write_line("y"),
            Builder::write_line("z"),
            Builder::write_line_str("Part two"),
            Builder::write_line_str("========"),
            Builder::write_line_str("Factorial of 0"),
            Builder::write_line_expr(Builder::method_call_literal("Program", "Factorial", vec![LiteralKind::Int(0)])),
            Builder::write_line_str("Factorial of 5"),
            Builder::write_line_expr(Builder::method_call_literal("Program", "Factorial", vec![LiteralKind::Int(5)])),
        ],
        is_static: true,
        return_ty: Type::Void
    });

    let aux_method = ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: "Aux".to_string(),
        params: vec![VarDecl { label: fresh_label(), var_name: "x".to_string(), ty: Type::Custom("int".to_string()), expr: None }],
        body: vec![
            Builder::decl_int("two", 2),
            Builder::decl_int_from_expr("sum", Builder::sum_vars("x", "two")),
            Builder::return_var("sum")
        ],
        is_static: true,
        return_ty: Type::Custom("int".to_string())
    });

    let factorial_method = ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: "Factorial".to_string(),
        params: vec![VarDecl { label: fresh_label(), var_name: "x".to_string(), ty: Type::Custom("int".to_string()), expr: None }],
        body: vec![
            Builder::if_then_else(
                Builder::binary_op(BinaryOperator::Eq, Builder::literal(LiteralKind::Int(0)), Builder::var_use("x")),
                vec![Builder::return_(Builder::literal(LiteralKind::Int(1)))],
                vec![Builder::return_(Builder::binary_op(
                        BinaryOperator::Mul,
                        Builder::var_use("x"),
                        Builder::method_call_expr("Program", "Factorial", vec![
                            Builder::binary_op(
                                BinaryOperator::Sub,
                                Builder::var_use("x"),
                                Builder::literal(LiteralKind::Int(1))
                            )
                        ])
                    ))]
            )
        ],
        is_static: true,
        return_ty: Type::Custom("int".to_string())
    });

    TopItem::ClassDecl(ClassDecl {
        label: fresh_label(),
        name: "Program".to_string(),
        items: vec![main_method, aux_method, factorial_method]
    })
}

pub fn variables() -> Program {
    Program {
        items: vec![
            program_class()
        ]
    }
}
