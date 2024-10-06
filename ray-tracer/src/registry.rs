use crate::shapes::Object;

pub struct Registry {
    objects: Vec<Object>,
    object_count: usize,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            object_count: 0,
        }
    }

    pub fn get_object(&self, id: usize) -> Option<&Object> {
        self.objects.iter().find(|obj| obj.id == id)
    }

    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut Object> {
        self.objects.iter_mut().find(|obj| obj.id == id)
    }

    pub fn next_object_id(&mut self) -> usize {
        let next_id = self.object_count;
        self.object_count += 1;
        next_id
    }

    pub fn add_object(&mut self, obj: Object) -> usize {
        self.objects.push(obj);
        self.objects.len()
    }
}
