use crate::support::camera::PerspectiveCamera;

pub trait Drawable {
    fn draw(&self, camera: &PerspectiveCamera);
}
