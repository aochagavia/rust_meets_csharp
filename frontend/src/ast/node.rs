use ast::*;

macro_rules! nodes {
    ( $( $x:ident ),* ) => {
        pub enum Node<'a> {
            $(
                $x(&'a $x),
            )*
        }

        $(
            impl DerivedFromNode for $x {
                fn unwrap<'a>(node: &'a Node) -> &'a $x {
                    match *node {
                        Node::$x(ref x) => x,
                        _ => unreachable!()
                    }
                }
            }
        )*
    }
}

nodes! {
    // Top level items
    ClassDecl,
    // Class items
    FieldDecl,
    MethodDecl,
    // Expressions
    FieldAccess,
    MethodCall,
    Identifier
}

impl<'a> Node<'a> {
    pub fn downcast<T>(&'a self) -> &'a T
    where T: DerivedFromNode {
        T::unwrap(self)
    }
}

pub trait DerivedFromNode {
    fn unwrap<'a>(node: &'a Node) -> &'a Self;
}
