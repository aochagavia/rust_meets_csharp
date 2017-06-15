use analysis::ClassInfo;
use ast;
use ir;

pub struct ProgramMetadata {
    pub classes: Vec<ClassInfo>,
    pub methods: Vec<ir::Method>
}

pub struct LoweringContext {
    pub metadata: ProgramMetadata
}

pub fn lower(p: &ast::Program) -> (ir::Program, ProgramMetadata) {
    // FIXME: implement real lowering

    let classes = vec![
        ClassInfo {
            superclass_id: None,
            name: "Console".to_string(),
            field_names: Vec::new(),
            methods: vec![0]
        }
    ];
    let entry_point = 0;
    let methods = vec![
        ir::Method {
            body: vec![
                ir::Statement::Expression(
                    ir::Expression::Intrinsic(
                        Box::new(
                            ir::Intrinsic::PrintLine(
                                ir::Expression::Literal(
                                    ir::Literal::String("Hello world!".to_string())
                                )
                            )
                        )
                    )
                )
            ]
        }
    ];

    (ir::Program { entry_point }, ProgramMetadata { classes, methods })
}

impl LoweringContext {
    fn lower_program(&mut self, p: &ast::Program) {
        for &ast::TopItem::ClassDecl(ref cd) in &p.items {
            self.lower_class_decl(cd);
        }
    }

    fn lower_class_decl(&mut self, c: &ast::ClassDecl) {

    }
}
