use std::collections::HashMap;

use cgmath::{Array, Vector3, Vector4};

use crate::{
    compute_shader_program::ComputeShaderProgram,
    geometry::Geometry,
    gl,
    shader_program::ShaderProgram,
    structures::vortex::Vortex,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self, random_inside_sphere, random_inside_unit_sphere, random_on_unit_sphere},
};

pub struct ActiveVorticies {
    pub shader_program: ShaderProgram,
    pub compute_program: ComputeShaderProgram,
    pub ssbo: u32,
    pub geometry: Geometry,
    pub mirror_number: usize,
    pub number_of_vorticies: usize,
    pub fading_enabled: bool,
    pub min_lifetime: f32,
    pub max_lifetime: f32,
    pub min_vorticity: f32,
    pub max_vorticity: f32,
}

impl ActiveVorticies {
    pub fn new(
        vorticies: Vec<Vortex>,
        min_lifetime: f32,
        max_lifetime: f32,
        min_vorticity: f32,
        max_vorticity: f32,
        mirror_number: usize,
    ) -> ActiveVorticies {
        let vortex_program = ShaderProgram::new(
            "Vortex",
            "resources/shaders/vortex.vert",
            "resources/shaders/vortex.frag",
            HashMap::new(),
        );
        let compute_program_vortex =
            ComputeShaderProgram::new("resources/shaders/vortex.comp", HashMap::new());

        //Expand the vortices to double the size
        let mut vorticies = vorticies.clone();
        vorticies.resize(
            (mirror_number + 1) * vorticies.len(),
            Vortex {
                ..Default::default()
            },
        );

        let ssbo = util::create_buffer();
        ActiveVorticies::load_data_to_ssbo(ssbo, &vorticies);

        let vao = util::create_vao();

        let geometry = Geometry::from_gltf("resources/models/arrow.glb", vao);

        let number_of_vorticies = vorticies.len();

        ActiveVorticies {
            shader_program: vortex_program,
            geometry,
            compute_program: compute_program_vortex,
            ssbo,
            mirror_number,
            number_of_vorticies,
            fading_enabled: true,
            min_lifetime,
            max_lifetime,
            min_vorticity,
            max_vorticity,
        }
    }

    pub fn new_random(
        number_of_vorticies: usize,
        min_lifetime: f32,
        max_lifetime: f32,
        min_vorticity: f32,
        max_vorticity: f32,
        mirror_number: usize,
    ) -> ActiveVorticies {
        let vorticies = ActiveVorticies::get_random_vorticies(
            number_of_vorticies,
            min_lifetime,
            max_lifetime,
            min_vorticity,
            max_vorticity,
        );

        ActiveVorticies::new(
            vorticies,
            min_lifetime,
            max_lifetime,
            min_vorticity,
            max_vorticity,
            mirror_number,
        )
    }

    fn get_random_vorticies(
        n: usize,
        min_lifetime: f32,
        max_lifetime: f32,
        min_vorticity: f32,
        max_vorticity: f32,
    ) -> Vec<Vortex> {
        (0..n)
            .map(|_| {
                let rand = rand::random::<f32>() * (max_lifetime - min_lifetime) + min_lifetime;

                let position = random_inside_unit_sphere();
                let vorticity = random_inside_sphere(min_vorticity, max_vorticity);
                Vortex {
                    position: position.extend(1.),
                    vorticity: vorticity.extend(0.),
                    lifetime: Vector4 {
                        x: rand,
                        y: rand,
                        z: 0.,
                        w: 0.,
                    },
                    ..Default::default()
                }
            })
            .collect()
    }

    fn load_data_to_ssbo(ssbo: u32, vorticies: &Vec<Vortex>) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (vorticies.len() * std::mem::size_of::<Vortex>()) as isize,
                vorticies.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 2, ssbo);
        }
    }

    pub fn with_fading(mut self, fading_enabled: bool) -> ActiveVorticies {
        self.fading_enabled = fading_enabled;
        self
    }
}

impl Drawable for ActiveVorticies {
    fn draw(&self, camera: &PerspectiveCamera) {
        return;
        self.shader_program.use_program();
        unsafe {
            self.shader_program
                .bind_uniform_matrix4fv("viewProjectionMatrix", &camera.view_proj_matrix[0][0]);
            self.shader_program.bind_uniform_4fv(
                "color",
                vec![Vector4 {
                    x: 1f32,
                    y: 0f32,
                    z: 0f32,
                    w: 1f32,
                }],
            );
            self.shader_program
                .bind_uniform_1ui("fading_enabled", self.fading_enabled as u32);
            self.geometry
                .draw_instanced(self.number_of_vorticies as i32);
        };
    }
}

impl Steppable for ActiveVorticies {
    fn step(&mut self, dt: f32, _camera: &PerspectiveCamera) {
        unsafe {
            let random_vector = Vector3::<f32> {
                x: rand::random::<f32>(),
                y: rand::random::<f32>(),
                z: rand::random::<f32>(),
            };
            self.compute_program.use_program();

            let _randoms_on_sphere = (0..(self.mirror_number * self.number_of_vorticies))
                .map(|_| random_on_unit_sphere())
                .collect::<Vec<Vector3<f32>>>();

            gl::Uniform1f(0, dt);
            gl::Uniform3fv(1, 1, random_vector.as_ptr());
            gl::Uniform1ui(2, self.mirror_number as u32);
            gl::Uniform1f(3, self.min_lifetime);
            gl::Uniform1f(4, self.max_lifetime);
            gl::Uniform1f(5, self.min_vorticity);
            gl::Uniform1f(6, self.max_vorticity);
            gl::DispatchCompute((self.number_of_vorticies / 256).max(1) as u32, 1, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }
}
