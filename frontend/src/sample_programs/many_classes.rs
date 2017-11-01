use ast::*;

fn empty_method(name: &str) -> ClassItem {
    ClassItem::MethodDecl(MethodDecl {
        label: fresh_label(),
        name: name.to_string(),
        params: Vec::new(),
        body: Vec::new(),
        is_static: true,
        return_ty: Type::Void
    })
}

fn empty_methods() -> Vec<ClassItem> {
    let amount = 3;
    let mut methods = Vec::with_capacity(amount);
    for method_count in 0..amount {
        let method_name = format!("Method{}", method_count);
        methods.push(empty_method(&method_name));
    }
    methods
}

fn classes() -> Vec<TopItem> {
    let amount = 1000;
    let mut classes = Vec::with_capacity(amount);
    for class_count in 0..amount {
        let class_name = format!("C{}", class_count);
        let class = TopItem::ClassDecl(ClassDecl {
            label: fresh_label(),
            name: class_name,
            items: empty_methods()
        });
        classes.push(class);
    }
    classes
}

fn main_class() -> TopItem {
    TopItem::ClassDecl(ClassDecl {
        label: fresh_label(),
        name: "Program".to_string(),
        items: vec![empty_method("Main")]
    })
}

pub fn many_classes() -> Program {
    let mut classes = classes();
    classes.push(main_class());
    Program {
        items: classes
    }
}
