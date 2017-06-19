use std::collections::HashMap;
use std::collections::hash_map::Entry;

use ast::*;
use super::{ClassInfo, IntrinsicInfo, ClassId, FieldId, MethodId, Type, TypeId, VarId};

pub struct QueryEngine<'a> {
    program: &'a Program,
    types: TypeMap
}

pub struct TypeMap {
    types: Vec<Type>,
    ids: HashMap<Type, usize>
}

impl TypeMap {
    pub fn get(&self, id: TypeId) -> Type {
        // Note: since we get a TypeId, it must be valid
        // You could get this wrong by generating the id from a different QueryEngine,
        // but even then it would just crash
        *self.types.get(id.0).unwrap()
    }

    pub fn get_id(&mut self, ty: Type) -> TypeId {
        match self.ids.entry(ty) {
            Entry::Vacant(e) => {
                // Intern the type
                let fresh_id = self.types.len();
                e.insert(fresh_id);
                self.types.push(ty);
                TypeId(fresh_id)
            }
            Entry::Occupied(e) => {
                TypeId(*e.get())
            }
        }
    }
}

impl Default for TypeMap {
    fn default() -> TypeMap {
        let types = vec![
            Type::Int,
            Type::String,
            Type::Void
        ];

        let mut ids = HashMap::new();
        for (id, ty) in types.iter().enumerate() {
            *ids.get_mut(ty).unwrap() = id;
        }

        TypeMap { types, ids }
    }
}

#[allow(unused_variables)]
impl<'a> QueryEngine<'a> {
    pub fn new(program: &'a Program) -> QueryEngine<'a> {
        // FIXME: populate the tables with type information so intrinsics
        // can be properly analyzed
        QueryEngine { program, types: TypeMap::default() }
    }

    pub fn types(&self) -> &TypeMap {
        &self.types
    }

    pub fn types_mut(&mut self) -> &mut TypeMap {
        &mut self.types
    }

    pub fn intrinsics() -> Vec<IntrinsicInfo> {
        // Note: this is a temporary hack. We may use it while there is only one intrinsic,\
        // but if more intrinsics are added this will need to be changed

        // Potential additional intrinsics that may be useful:
        // * Console.Write
        // * Console.ReadLine
        vec![IntrinsicInfo]
    }

    pub fn methods(&self) -> Vec<&'a MethodDecl> {
        unimplemented!()
    }

    pub fn query_entry_point(&mut self) -> MethodId {
        unimplemented!()
    }

    pub fn query_class_info(&mut self, class_id: ClassId) -> Vec<ClassInfo> {
        unimplemented!()
    }

    pub fn query_field(&mut self, class_id: ClassId, field_name: &str) -> FieldId {
        unimplemented!()
    }

    pub fn query_method(&mut self, class_id: ClassId, method_name: &str) -> Option<MethodId> {
        unimplemented!()
    }

    pub fn query_param_types(&mut self, method_id: MethodId) -> Vec<TypeId> {
        unimplemented!()
    }

    pub fn query_constructor(&mut self, class_id: ClassId) -> MethodId {
        unimplemented!()
    }

    pub fn query_class(&mut self, class_name: &str) -> Option<ClassId> {
        unimplemented!()
    }

    pub fn query_var_decl(&mut self, identifier: Label) -> VarId {
        // The label may correspong to a VarAssign, VarDecl or VarRead
        unimplemented!()
    }

    pub fn query_var(&mut self, identifier: Label) -> VarId {
        unimplemented!()
    }

    pub fn query_var_type(&mut self, identifier: VarId) -> TypeId {
        unimplemented!()
    }

    pub fn query_parent_method(&mut self, node: Label) -> MethodId {
        // Note: should work for statements and expressions. Panics otherwise.
        unimplemented!()
    }

    pub fn query_return_type(&mut self, method: MethodId) -> TypeId {
        unimplemented!()
    }

    pub fn query_is_static(&mut self, method: MethodId) -> bool {
        unimplemented!()
    }

    /// Returns the type of an expression, or `None` in case the label
    /// corresponds to another kind of node. Note: references to undefined
    /// variables have no type.
    pub fn query_expr_type(&mut self, expr: Label) -> Option<TypeId> {
        unimplemented!()
    }
}
