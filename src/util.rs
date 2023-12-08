use std::f32::consts::PI;

use cgmath::Vector3;
use easy_gltf::load;
use rand::distributions::{Distribution, Uniform};

use crate::gl;

pub fn create_buffer() -> u32 {
    unsafe {
        let mut vao = 0;
        gl::GenBuffers(1, &mut vao);
        assert_ne!(vao, 0);

        vao
    }
}

pub fn create_vao() -> u32 {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        vao
    }
}

pub fn random_inside_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    random_on_unit_sphere() * Uniform::new(0., 1.).sample(&mut rng)
}

pub fn random_inside_sphere(inner_radius: f32, outer_radius: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    random_on_unit_sphere() * Uniform::new(inner_radius, outer_radius).sample(&mut rng)
}

pub fn random_on_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let phi = Uniform::new(0., 2. * PI).sample(&mut rng);

    let z = Uniform::new(-1., 1.).sample(&mut rng);
    let x = phi.cos() * (1.0f32 - z * z).sqrt();
    let y = phi.sin() * (1.0f32 - z * z).sqrt();

    Vector3 { x, y, z }
}

pub fn get_vertices_and_normals_from_gltf(model_path: &str) -> Vec<(Vector3<f32>, Vector3<f32>)> {
    let scenes = load(model_path).expect("Failed to load gltf file");
    let first_scene = scenes.into_iter().next().expect("No scenes in gltf file");

    first_scene
        .models
        .iter()
        .flat_map(|m| m.vertices().clone())
        .map(|v| (v.position, v.normal))
        .collect::<Vec<_>>()
}
