use ast::*;

fn program_class() -> TopItem {
    let main_method = ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: "Main".to_string(),
        params: vec![VarDecl { label: fresh_label(), var_name: "args".to_string(), ty: Type::Array(Box::new(Type::Custom("String".to_string()))), expr: None }],
        body: vec![
            Statement::Expression(Expression::MethodCall(MethodCall {
                label: fresh_label(),
                target: Box::new(Expression::Identifier(Identifier { name: "Console".to_string(), label: fresh_label() })),
                method_name: "WriteLine".to_string(),
                args: vec![Expression::Literal(Literal { label: fresh_label(), kind: LiteralKind::String("Hello world!".to_string()) })]
            }))
        ],
        is_static: true,
        return_ty: Type::Void
    });

    TopItem::ClassDecl(ClassDecl {
        label: fresh_label(),
        name: "Program".to_string(),
        items: vec![main_method]
    })
}

pub fn hello_world() -> Program {
    Program {
        items: vec![
            program_class()
        ]
    }
}
