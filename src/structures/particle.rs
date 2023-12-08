use cgmath::Vector4;

#[repr(C)]
pub struct Particle {
    pub position: Vector4<f32>,
    pub lifetime: Vector4<f32>,
    pub velocity: Vector4<f32>,
}
