use crate::shapes::Object;

pub struct Registry {
    objects: Vec<Object>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn get_object(&self, id: usize) -> Option<&Object> {
        self.objects.get(id)
    }

    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut Object> {
        self.objects.get_mut(id)
    }

    pub fn next_object_id(&self) -> usize {
        self.objects.len()
    }

    pub fn add_object(&mut self, obj: Object) -> usize {
        self.objects.push(obj);
        self.objects.len()
    }
}
