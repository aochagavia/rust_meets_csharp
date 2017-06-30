use ast::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClassDecl(pub(crate) Label);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeUse(pub(crate) Label);

#[derive(Clone, Copy)]
pub struct Expression(pub(crate) Label);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MethodDecl(pub(crate) Label);

#[derive(Clone, Copy)]
pub struct MethodUse(pub(crate) Label);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct VarDecl(pub(crate) Label);

#[derive(Clone, Copy)]
pub struct VarUse(pub(crate) Label);

impl Label {
    pub fn assert_as_class_decl(self) -> ClassDecl {
        ClassDecl(self)
    }

    pub fn assert_as_type_use(self) -> TypeUse {
        TypeUse(self)
    }

    pub fn assert_as_method_decl(self) -> MethodDecl {
        MethodDecl(self)
    }

    pub fn assert_as_method_use(self) -> MethodUse {
        MethodUse(self)
    }

    pub fn assert_as_var_decl(self) -> VarDecl {
        VarDecl(self)
    }

    pub fn assert_as_var_use(self) -> VarUse {
        VarUse(self)
    }
}

macro_rules! impl_as_label {
    ( $( $x:ty ),* ) => {
        $(
            impl $x {
                pub fn as_label(self) -> Label {
                    self.0
                }
            }
        )*
    }
}

impl_as_label!(ClassDecl, Expression, MethodDecl, MethodUse, VarDecl, VarUse);
