use std::collections::HashMap;
use std::usize;

use analysis::labels;
use ast;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

/// Represents the type of an expression
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    Bool,
    Int,
    String,
    Console,
    Array(TypeId),
    Void,
    Class(labels::ClassDecl)
}

impl Type {
    pub fn class_decl(&self) -> labels::ClassDecl {
        match self {
            &Type::Class(cd) => cd,
            _ => panic!("Type was not a Class type")
        }
    }
}

pub struct TypeMap {
    types: Vec<Type>,
    ids: HashMap<Type, usize>,
}

impl TypeMap {
    pub fn get(&self, id: TypeId) -> Type {
        // Note: since we get a TypeId, we assume it is valid
        // You could get this wrong by generating the id from a different QueryEngine,
        // but even then it would just crash
        if id == self.any_ty() {
            panic!("Attempted to get the type corresponding to any type");
        }

        self.types[id.0]
    }

    pub fn get_id(&mut self, ty: Type) -> TypeId {
        let types = &mut self.types;
        let inner_id = *self.ids.entry(ty).or_insert_with(|| {
            let fresh_id = types.len();
            types.push(ty);
            fresh_id
        });

        TypeId(inner_id)
    }

    pub fn any_ty(&self) -> TypeId {
        TypeId(usize::MAX)
    }

    pub fn string_ty(&self) -> TypeId {
        TypeId(1)
    }

    pub fn int_ty(&self) -> TypeId {
        TypeId(0)
    }

    pub fn void_ty(&self) -> TypeId {
        TypeId(2)
    }

    pub fn bool_ty(&self) -> TypeId {
        TypeId(3)
    }

    pub fn console_ty(&self) -> TypeId {
        TypeId(4)
    }

    pub fn unify(&self, ty1: TypeId, ty2: TypeId) -> bool {
        self.any_ty() == ty1 // One of both types are null
        || self.any_ty() == ty2
        || ty1 == ty2 // Both types are equal
    }

    pub fn get_from_class_name(&mut self, name: &str, decls: &HashMap<&str, &ast::ClassDecl>) -> TypeId {
        match decls.get(name) {
            Some(ref class) => {
                let decl = class.label.assert_as_class_decl();
                self.get_id(Type::Class(decl))
            }
            None => {
                panic!("Class decl not found for `{}`", name);
            }
        }
    }

    pub fn get_from_ast_ty(&mut self, ast_ty: &ast::Type, decls: &HashMap<&str, &ast::ClassDecl>) -> TypeId {
        match ast_ty {
            &ast::Type::Array(ref inner_ty) => {
                // Recursively get the inner type
                let inner_ty_id = self.get_from_ast_ty(inner_ty, decls);
                self.get_id(Type::Array(inner_ty_id))
            }
            &ast::Type::Custom(ref ty_name) => {
                // See if there is a class that matches the custom type
                let ty_name: &str = ty_name;
                match ty_name {
                    "bool" => self.bool_ty(),
                    "int" => {
                        self.int_ty()
                    }
                    "string" | "String" => {
                        self.string_ty()
                    }
                    "Console" => {
                        self.console_ty()
                    }
                    class_name => {
                        // Not a builtin type. We need to find the decl for this class. We assume it exists
                        self.get_from_class_name(class_name, decls)
                    }
                }
            }
            &ast::Type::Void => {
                self.void_ty()
            }
        }
    }
}

impl Default for TypeMap {
    fn default() -> TypeMap {
        let types = vec![
            Type::Bool,
            Type::Int,
            Type::String,
            Type::Void,
            Type::Console
        ];

        let mut ids = HashMap::new();
        for (id, &ty) in types.iter().enumerate() {
            ids.insert(ty, id);
        }

        ids.insert(Type::Console, 3);

        TypeMap { types, ids }
    }
}
