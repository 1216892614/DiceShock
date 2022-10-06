pub(crate) struct Model {
    models: Vec<tobj::Model>,
    obj_materials: Vec<tobj::Material>,
}

impl Model {
    pub(crate) fn new(models: Vec<tobj::Model>, obj_materials: Vec<tobj::Material>) -> Self {
        Self {
            models,
            obj_materials,
        }
    }
}
