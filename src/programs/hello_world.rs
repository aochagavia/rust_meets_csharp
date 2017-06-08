use ast::*;

fn console_class() -> TopItem {
    // Provide two methods
    fn create_method(name: &str) -> ClassItem {
        ClassItem::MethodDecl(MethodDecl {
            label: fresh_label(),
            name: name.to_string(),
            params: vec![Param { label: fresh_label(), name: "arg".to_string(), ty: Type::String }],
            body: Vec::new(),
            is_static: true,
            return_ty: Type::Void
        })
    }

    TopItem::ClassDecl(ClassDecl {
        label: fresh_label(),
        name: "Console".to_string(),
        superclass: None,
        items: vec![
            create_method("Write"),
            create_method("WriteLine"),
            ClassItem::FieldDecl(FieldDecl { label: fresh_label(), name: "MyField".to_string(), ty: Type::Int, assignment: None })
        ]
    })
}

fn program_class() -> TopItem {
    let main_method = ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: "Main".to_string(),
        params: vec![Param { label: fresh_label(), name: "args".to_string(), ty: Type::Array(Box::new(Type::String)) }],
        body: vec![
            Statement::Expression(Expression::MethodCall(MethodCall {
                label: fresh_label(),
                target: "Console".to_string(),
                method_name: "WriteLine".to_string(),
                args: vec![Expression::Literal(Literal::String("Hello world!".to_string()))]
            }))
        ],
        is_static: true,
        return_ty: Type::Void
    });

    TopItem::ClassDecl(ClassDecl {
        label: fresh_label(),
        name: "Program".to_string(),
        superclass: None,
        items: vec![main_method]
    })
}

pub fn hello_world() -> Program {
    Program {
        items: vec![
            console_class(),
            program_class()
        ]
    }
}
