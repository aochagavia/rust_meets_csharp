use ast::*;

fn console_class() -> TopItem {
    // Provide two methods
    fn create_method(name: &str) -> ClassItem {
        ClassItem::MethodDecl {
            name: name.into(),
            params: vec![("arg".into(), Type::String)],
            body: Vec::new(),
            return_ty: Type::Void
        }
    }

    TopItem::ClassDecl {
        name: "Console".into(),
        inherits_from: None,
        items: vec![
            create_method("Write").into(),
            create_method("WriteLine").into(),
            ClassItem::FieldDecl { name: "MyField".into(), ty: Type::Int, assignment: None }.into()
        ]
    }
}

fn program_class() -> TopItem {
    let main_method = ClassItem::MethodDecl {
        name: "Main".into(),
        params: vec![("args".into(),
        Type::Array(Box::new(Type::String)))],
        body: vec![
            Statement::Expression(Expression::MethodCall {
                target: "Console".into(),
                method_name: "WriteLine".into(),
                params: vec![Expression::Literal(Literal::String("Hello world!".into()))]
            }).into()
        ],
        return_ty: Type::Void
    };

    TopItem::ClassDecl {
        name: "Program".into(),
        inherits_from: None,
        items: vec![main_method.into()]
    }
}

pub fn hello_world() -> Program {
    Program {
        items: vec![
            console_class().into(),
            program_class().into()
        ]
    }
}
