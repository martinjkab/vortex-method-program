use std::{collections::HashMap, fs};

use cgmath::{Matrix, Matrix4, Vector3};

use crate::{
    geometry::Geometry,
    gl,
    shader_program::ShaderProgram,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self},
};

pub struct BoundaryMesh {
    pub program: ShaderProgram,
    pub geometry: Geometry,
    pub vao: u32,
    pub hover: bool,
}

impl BoundaryMesh {
    pub fn new(model_path: &str) -> BoundaryMesh {
        let program = ShaderProgram::new(
            "Boundary Mesh",
            "resources/shaders/boundary_mesh.vert",
            "resources/shaders/boundary_mesh.frag",
            HashMap::new(),
        );

        let vao = util::create_vao();

        let geometry = Geometry::from_gltf(model_path, vao);

        BoundaryMesh {
            program,
            geometry,
            vao,
            hover: false,
        }
    }
}

impl Drawable for BoundaryMesh {
    fn draw(&self, camera: &PerspectiveCamera) {
        self.program.use_program();
        let model_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)).transpose();
        let mvp = model_matrix * camera.view_proj_matrix;
        unsafe {
            self.program.bind_uniform_matrix4fv("mvp", &mvp[0][0]);
        }
        self.geometry.draw();
    }
}

impl Steppable for BoundaryMesh {}
