mod query_engine;

pub struct ClassInfo {
    pub superclass_id: Option<usize>,
    pub name: String,
    pub field_names: Vec<String>,

    // FIXME: should we make a distinction between static and dynamic methods?
    pub methods: Vec<usize>,
}
