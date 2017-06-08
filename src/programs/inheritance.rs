use ast::*;

fn parent_class() -> TopItem {
    TopItem::ClassDecl(ClassDecl {
        name: "Parent".into(),
        superclass: None,
        items: vec![
            ClassItem::MethodDecl(MethodDecl { name: "ParentMethod".into(), params: Vec::new(), body: Vec::new(), return_ty: Type::Void }).into(),
            ClassItem::FieldDecl(FieldDecl { name: "ParentField".into(), ty: Type::Int, assignment: None }).into()
        ]
    })
}

fn child_class() -> TopItem {
    TopItem::ClassDecl(ClassDecl {
        name: "Child".into(),
        superclass: Some("Parent".into()),
        items: vec![
            ClassItem::MethodDecl(MethodDecl { name: "ChildMethod".into(), params: Vec::new(), body: Vec::new(), return_ty: Type::Void }).into(),
            ClassItem::FieldDecl(FieldDecl { name: "ChildField".into(), ty: Type::Int, assignment: None }).into()
        ]
    })
}

fn program_class() -> TopItem {
    let main_method = ClassItem::MethodDecl(MethodDecl {
        name: "Main".into(),
        params: vec![("args".into(),
        Type::Array(Box::new(Type::String)))],
        body: vec![
            Statement::VarDecl {
                var_name: "child".into(),
                ty: Type::Custom("Child".into()),
                expr: Some(Expression::New { class_name: "Child".into(), params: Vec::new() })
            }.into()
        ],
        return_ty: Type::Void
    });

    TopItem::ClassDecl(ClassDecl {
        name: "Program".into(),
        superclass: None,
        items: vec![main_method.into()]
    })
}

pub fn inheritance() -> Program {
    Program {
        items: vec![
            parent_class().into(),
            child_class().into(),
            program_class().into()
        ]
    }
}