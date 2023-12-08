use cgmath::{Point3};

use crate::structures::ray::Ray;

pub trait Collider {
    fn collides_with_ray(&self, point: &Ray) -> Option<Point3<f32>>;
}
