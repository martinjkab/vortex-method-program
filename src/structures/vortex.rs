use cgmath::Vector4;
use std::fmt::{Debug, Formatter};

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vortex {
    pub position: Vector4<f32>,
    pub normal: Vector4<f32>,
    pub vorticity: Vector4<f32>,
    pub lifetime: Vector4<f32>,
}

impl Debug for Vortex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vortex")
            .field("position", &self.position)
            .field("normal", &self.normal)
            .field("vorticity", &self.vorticity)
            .field("lifetime", &self.lifetime)
            .finish()
    }
}

impl Default for Vortex {
    fn default() -> Self {
        Self {
            position: Vector4::new(0., 0., 0., 1.),
            normal: Vector4::new(0., 0., 0., 1.),
            vorticity: Vector4::new(0., 0., 0., 0.),
            lifetime: Vector4::new(0., 0., 0., 0.),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BoundaryInfo {
    pub active_index: i32,
    pub active_count: i32,
    pub count: i32,
}
