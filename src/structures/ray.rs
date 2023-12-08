use std::fmt::Debug;

use cgmath::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Debug for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ray")
            .field("origin", &self.origin)
            .field("direction", &self.direction)
            .finish()
    }
}
