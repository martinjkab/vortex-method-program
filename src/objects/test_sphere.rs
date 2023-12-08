use std::{collections::HashMap, fs};

use cgmath::{Matrix, Matrix4, Vector3};

use crate::{
    compute_shader_program::ComputeShaderProgram,
    geometry::Geometry,
    gl,
    shader_program::ShaderProgram,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self},
};

pub struct TestSphere {
    pub program: ShaderProgram,
    pub geometry: Geometry,
    pub compute_program: ComputeShaderProgram,
    pub vao: u32,
    pub hover: bool,
}

impl TestSphere {
    pub fn new(model_path: &str) -> TestSphere {
        let program = ShaderProgram::new(
            "Test Sphere",
            "resources/shaders/test_sphere.vert",
            "resources/shaders/test_sphere.frag",
            HashMap::from([(
                "get_velocity",
                fs::read_to_string("resources/gpu_methods/get_velocity.glsl")
                    .unwrap()
                    .as_str(),
            )]),
        );

        let vao = util::create_vao();

        let geometry = Geometry::from_gltf(model_path, vao);

        let compute_program =
            ComputeShaderProgram::new("resources/shaders/cube.comp", HashMap::new());

        TestSphere {
            program,
            geometry,
            compute_program,
            vao,
            hover: false,
        }
    }
}

impl Drawable for TestSphere {
    fn draw(&self, camera: &PerspectiveCamera) {
        self.program.use_program();
        let model_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)).transpose();
        let mvp = model_matrix * camera.view_proj_matrix;
        unsafe {
            self.program.bind_uniform_matrix4fv("mvp", &mvp[0][0]);
            self.program
                .bind_uniform_matrix4fv("model", &model_matrix[0][0]);

            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::BLEND);
        }
        self.geometry.draw();
    }
}

impl Steppable for TestSphere {
    fn step(&mut self, dt: f32, _camera: &PerspectiveCamera) {
        unsafe {
            self.compute_program.use_program();
            gl::Uniform1f(0, dt);
            gl::Uniform1f(1, 1f32);
            gl::Uniform1f(2, 0.1f32);
            gl::Uniform1f(3, 1.);
            gl::DispatchCompute(1_u32, 1, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }
}
