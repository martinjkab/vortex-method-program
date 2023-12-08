use std::collections::HashMap;

use cgmath::num_traits::Pow;
use cgmath::{InnerSpace, Matrix, Matrix4, Point3, Vector4};

use crate::structures::ray::Ray;
use crate::traits::collider::Collider;
use crate::{
    compute_shader_program::ComputeShaderProgram,
    geometry::Geometry,
    gl,
    shader_program::ShaderProgram,
    structures::cube::Cube,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self},
};

pub struct CubeGeometry {
    pub program: ShaderProgram,
    pub geometry: Geometry,
    pub compute_program: ComputeShaderProgram,
    pub ssbo: u32,
    pub vao: u32,
    pub hover: bool,
}

impl CubeGeometry {
    pub fn new() -> CubeGeometry {
        let program = ShaderProgram::new(
            "Cube",
            "resources/shaders/cube.vert",
            "resources/shaders/cube.frag",
            HashMap::new(),
        );

        let vao = util::create_vao();

        let geometry = Geometry::from_gltf("resources/models/gomb.glb", vao);

        let compute_program =
            ComputeShaderProgram::new("resources/shaders/cube.comp", HashMap::new());

        let ssbo = util::create_buffer();
        CubeGeometry::load_data_to_ssbo(
            ssbo,
            Cube {
                position: Vector4 {
                    x: 0f32,
                    y: 0f32,
                    z: 0f32,
                    w: 1f32,
                },
                velocity: Vector4 {
                    x: 0f32,
                    y: 0f32,
                    z: 0f32,
                    w: 0f32,
                },
                color: Vector4 {
                    x: 1f32,
                    y: 0f32,
                    z: 0f32,
                    w: 1f32,
                },
            },
        );

        CubeGeometry {
            program,
            geometry,
            compute_program,
            ssbo,
            vao,
            hover: false,
        }
    }

    fn load_data_to_ssbo(ssbo: u32, cube: Cube) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (std::mem::size_of::<Cube>()) as isize,
                &cube as *const Cube as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 3, ssbo);
        }
    }

    pub fn data(&self) -> Cube {
        let mut cube: Cube = Cube {
            position: Vector4 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
                w: 1f32,
            },
            velocity: Vector4 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
                w: 1f32,
            },
            color: Vector4 {
                x: 0f32,
                y: 1f32,
                z: 0f32,
                w: 1f32,
            },
        };
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.ssbo);
            gl::GetBufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                0,
                (std::mem::size_of::<Cube>()) as isize,
                &mut cube as *mut Cube as *mut std::ffi::c_void,
            )
        };

        cube
    }
}

impl Default for CubeGeometry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for CubeGeometry {
    fn draw(&self, camera: &PerspectiveCamera) {
        self.program.use_program();
        let cube = self.data();
        let model_matrix = Matrix4::from_translation(cube.position.truncate()).transpose();
        let mvp = model_matrix * camera.view_proj_matrix;
        self.program.bind_uniform_1ui("hover", self.hover as u32);
        unsafe {
            self.program.bind_uniform_matrix4fv("mvp", &mvp[0][0]);
        }
        self.geometry.draw();
    }
}

impl Steppable for CubeGeometry {
    fn step(&mut self, dt: f32, camera: &PerspectiveCamera) {
        let mouse_ray = camera.unproject_ray();

        let point = self.collides_with_ray(&mouse_ray);
        self.hover = point.is_some();
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

impl Collider for CubeGeometry {
    //The ray is already in world space
    fn collides_with_ray(&self, ray: &Ray) -> Option<Point3<f32>> {
        let cube = self.data();
        let sphere_center = Point3::new(cube.position.x, cube.position.y, cube.position.z);
        let sphere_radius = 1f32;
        let oc = ray.origin - sphere_center;
        let b = oc.dot(ray.direction);
        let c = oc.magnitude2() - sphere_radius.pow(2);
        let discriminant: f32 = b.pow(2) - c;
        if discriminant < 0f32 {
            println!("discriminant < 0");
            return None;
        }
        let t = -b - discriminant.sqrt();
        if t < 0f32 {
            println!("t < 0");
            return None;
        }
        let point = ray.origin + ray.direction * t;
        Some(point)
    }
}
