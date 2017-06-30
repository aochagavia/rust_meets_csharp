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
    ClassDecl,
    MethodDecl
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
