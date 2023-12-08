use cgmath::Vector4;

#[repr(C)]
pub struct Cube {
    pub position: Vector4<f32>,
    pub velocity: Vector4<f32>,
    pub color: Vector4<f32>,
}
