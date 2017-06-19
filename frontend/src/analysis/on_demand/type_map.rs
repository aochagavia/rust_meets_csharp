use std::collections::HashMap;
use analysis::{Type, TypeId};

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
        let types = &mut self.types;
        let inner_id = *self.ids.entry(ty).or_insert_with(|| {
            let fresh_id = types.len();
            types.push(ty);
            fresh_id
        });

        TypeId(inner_id)
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
