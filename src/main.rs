mod ast;

use ast::*;

fn main() {
}

fn console_class() -> TopItem {
    // Provide two methods
    fn create_method(name: &str) -> ClassItem {
        ClassItem::MethodDecl { name: name.into(), params: vec![("arg".into(), Type::String)], body: Vec::new(), return_ty: Type::Void }
    }

    TopItem::ClassDecl {
        name: "Console".into(),
        inherits_from: None,
        items: vec![
            create_method("Write"),
            create_method("WriteLine"),
            ClassItem::FieldDecl { name: "MyField".into(), ty: Type::Int, assignment: None }
        ]
    }
}

fn hello_world() -> Program {
    let main_method = ClassItem::MethodDecl {
        name: "Main".into(),
        params: vec![("args".into(),
        Type::Array(Box::new(Type::String)))],
        body: vec![
            Statement::Expression(Expression::StaticMethodCall {
                class_name: "Console".into(),
                method_name: "WriteLine".into(),
                params: vec![Expression::Literal(Literal::String("Hello world!".into()))]
            })
        ],
        return_ty: Type::Void
    };
    let main_class = TopItem::ClassDecl { name: "Program".into(), inherits_from: None, items: vec![main_method] };
    Program {
        items: vec![
            console_class(),
            main_class
        ]
    }
}
