use crate::{
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, object::Object, steppable::Steppable},
};

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Object + 'static) -> &mut Self {
        self.objects.push(Box::new(object));
        self
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Scene {
    fn draw(&self, camera: &PerspectiveCamera) {
        for object in &self.objects {
            object.draw(camera);
        }
    }
}

impl Steppable for Scene {
    fn step(&mut self, dt: f32, camera: &PerspectiveCamera) {
        for object in &mut self.objects {
            object.step(dt, camera);
        }
    }
}
