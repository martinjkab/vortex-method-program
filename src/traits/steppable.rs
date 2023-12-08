use crate::support::camera::PerspectiveCamera;

pub trait Steppable {
    fn step(&mut self, dt: f32, camera: &PerspectiveCamera) {}
}
