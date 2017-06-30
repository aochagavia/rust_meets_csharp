use ast::*;

macro_rules! nodes {
    ( $( $x:ident ),* ) => {
        pub enum Node {
            $(
                $x($x),
            )*
        }

        impl Node {
            $(
                #[allow(non_snake_case)]
                pub fn $x(&self) -> &$x {
                    match *self {
                        Node::$x(ref x) => x,
                        _ => unreachable!()
                    }
                }
            )*
        }
    }
}

nodes! {
    ClassDecl,
    MethodDecl
}

// FIXME: this macro generates CamelCase function names, which don't follow
// Rust convention
