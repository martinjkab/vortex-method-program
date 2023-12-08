use std::{collections::HashMap, fs, path::Path};

use cgmath::{Vector3, Vector4};

use crate::{
    compute_shader_program::ComputeShaderProgram,
    geometry::Geometry,
    gl,
    shader_program::ShaderProgram,
    structures::particle::Particle,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self, random_inside_unit_sphere, random_on_unit_sphere},
};

use super::texture::Texture;

pub struct Particles {
    pub shader: ShaderProgram,
    pub compute_shader: ComputeShaderProgram,
    pub sorting_compute_shader: ComputeShaderProgram,
    pub geometry: Geometry,
    pub ssbo: u32,
    pub number_of_particles: usize,
    pub resetting_enabled: bool,
    pub fading_enabled: bool,
    pub texture: Texture,
}

impl Particles {
    pub fn new(particles: Vec<Particle>) -> Particles {
        let shader = ShaderProgram::new(
            "Particles",
            "resources/shaders/particle.vert",
            "resources/shaders/particle.frag",
            HashMap::new(),
        );
        let compute_shader = ComputeShaderProgram::new(
            "resources/shaders/particle.comp",
            HashMap::from([(
                "get_velocity",
                fs::read_to_string("resources/gpu_methods/get_velocity.glsl")
                    .unwrap()
                    .as_str(),
            )]),
        );
        let sorting_compute_shader =
            ComputeShaderProgram::new("resources/shaders/particle_sort.comp", HashMap::new());

        let ssbo = util::create_buffer();
        Particles::load_data_to_ssbo_particles(ssbo, &particles);

        let vao = util::create_vao();
        unsafe {
            let texture = Texture::new();
            texture
                .load(Path::new("resources/textures/smoke.png"))
                .unwrap();
            Particles {
                shader,
                compute_shader,
                sorting_compute_shader,
                ssbo,
                geometry: Geometry::new_square(vao),
                number_of_particles: particles.len(),
                resetting_enabled: true,
                fading_enabled: true,
                texture,
            }
        }
    }

    pub fn new_inside_unit_sphere(n: usize) -> Particles {
        Particles::new(Particles::get_random_particles_inside_unit_sphere(n))
    }

    pub fn new_on_unit_sphere(n: usize) -> Particles {
        Particles::new(Particles::get_random_particles_on_unit_sphere(n))
    }

    fn load_data_to_ssbo_particles(ssbo: u32, particles: &Vec<Particle>) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (particles.len() * std::mem::size_of::<Particle>()) as isize,
                particles.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, ssbo);
        }
    }

    fn get_random_particles_inside_unit_sphere(n: usize) -> Vec<Particle> {
        (0..n)
            .map(|_| {
                let r = rand::random::<f32>() * 10.;
                let position =
                    Vector3::new(0.7f32, 0.4f32, 0.0f32) + random_inside_unit_sphere() * 0.1f32;
                // let position = random_inside_unit_sphere(); // * 0.1f32;
                Particle {
                    position: Vector4 {
                        x: position.x,
                        y: position.y,
                        z: position.z,
                        w: 1.,
                    },
                    lifetime: Vector4 {
                        x: r,
                        y: r,
                        z: 0f32,
                        w: 0f32,
                    },
                    velocity: Vector4 {
                        x: 0f32,
                        y: 0f32,
                        z: 0f32,
                        w: 0f32,
                    },
                }
            })
            .collect()
    }

    fn get_random_particles_on_unit_sphere(n: usize) -> Vec<Particle> {
        (0..n)
            .map(|_| {
                let r = rand::random::<f32>() * 10.;
                let position = random_on_unit_sphere();
                Particle {
                    position: Vector4 {
                        x: position.x,
                        y: position.y,
                        z: position.z,
                        w: 1.,
                    },
                    lifetime: Vector4 {
                        x: r,
                        y: r,
                        z: 0f32,
                        w: 0f32,
                    },
                    velocity: Vector4 {
                        x: 0f32,
                        y: 0f32,
                        z: 0f32,
                        w: 0f32,
                    },
                }
            })
            .collect()
    }

    pub fn with_resetting(mut self, resetting_enabled: bool) -> Particles {
        self.resetting_enabled = resetting_enabled;
        self
    }

    pub fn with_fading(mut self, fading_enabled: bool) -> Particles {
        self.fading_enabled = fading_enabled;
        self
    }
}

impl Drawable for Particles {
    fn draw(&self, camera: &PerspectiveCamera) {
        self.shader.use_program();
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
            self.shader
                .bind_uniform_matrix4fv("viewProjectionMatrix", &camera.view_proj_matrix[0][0]);
            self.shader
                .bind_uniform_1ui("fading_enabled", self.fading_enabled as u32);
            // self.shader.bind_uniform_4fv(
            //     "color",
            //     vec![Vector4 {
            //         x: 1f32,
            //         y: 1f32,
            //         z: 1f32,
            //         w: 1f32,
            //     }],
            // );
            self.texture.bind();
            self.geometry
                .draw_instanced(self.number_of_particles as i32);
        };
    }
}

impl Steppable for Particles {
    fn step(&mut self, dt: f32, _camera: &PerspectiveCamera) {
        unsafe {
            let random_vector = Vector3::<u32> {
                x: rand::random::<u32>(),
                y: rand::random::<u32>(),
                z: rand::random::<u32>(),
            };
            self.compute_shader.use_program();
            gl::Uniform1f(0, dt);
            gl::Uniform3uiv(1, 1, &random_vector[0]);
            gl::Uniform1ui(2, self.resetting_enabled as u32);
            gl::DispatchCompute(self.number_of_particles as u32, 1, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

            // for _i in 0..1024 {
            //     self.sorting_compute_shader.use_program();
            //     gl::UniformMatrix4fv(0, 1, false as u8, &camera.view_proj_matrix[0][0]);
            //     gl::DispatchCompute((self.number_of_particles as f32 / 2.) as u32, 1, 1);
            //     gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            // }
        }
    }
}
