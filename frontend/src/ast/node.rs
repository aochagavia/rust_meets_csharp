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

// Note: when adding structs to this list we need to make sure to modify the visitor as well
// Otherwise they won't be collected in the node map
nodes! {
    // Top level items
    ClassDecl,
    // Class items
    FieldDecl,
    MethodDecl,
    // Statements
    VarDecl,
    // Expressions
    FieldAccess, // Right now, this is useless... Since there is no way of assigning a value to a field
    MethodCall,
    Identifier,
    BinaryOp,
    Literal,
    New,
    This
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
