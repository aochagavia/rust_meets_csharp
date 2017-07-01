use ast::*;

macro_rules! nodes {
    ( $( $x:ident ),* ) => {
        pub enum Node {
            $(
                $x($x),
            )*
        }

        $(
            impl DerivedFromNode for $x {
                fn unwrap(node: &Node) -> &$x {
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

impl Node {
    pub fn downcast<T>(&self) -> &T
    where T: DerivedFromNode {
        T::unwrap(self)
    }
}

pub trait DerivedFromNode {
    fn unwrap(node: &Node) -> &Self;
}

// FIXME: this macro generates CamelCase function names, which don't follow
// Rust convention
