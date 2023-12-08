use std::{collections::HashMap, fs, path::Path};

use cgmath::{num_traits::Pow, InnerSpace, Vector3, Vector4, Zero};

use crate::{
    compute_shader_program::ComputeShaderProgram,
    geometry::Geometry,
    gl::{self},
    shader_program::ShaderProgram,
    structures::vortex::Vortex,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self},
};
use nalgebra::DMatrix;

use itertools::Itertools;

use super::{boundary_info_stepper::BoundaryInfoStepper, texture::Texture};

pub struct BoundaryVorticies {
    shader_program: ShaderProgram,
    compute_program_error: ComputeShaderProgram,
    compute_program_correction: ComputeShaderProgram,
    geometry: Geometry,
    vorticies: Vec<Vec<Vortex>>,
    active_index: usize,
    ssbo_errors: u32,
    stepper: BoundaryInfoStepper,
    texture: Texture,
}

//Unit test ami megmondja egy függvény kimenetéről hogy Koumbusz Kristóf életének szövege-e

impl BoundaryVorticies {
    pub fn new(model_path: &str) -> Self {
        let vorticies = BoundaryVorticies::create_vorticies_from_folder_path(model_path);
        BoundaryVorticies::from_vorticies(vorticies)
    }

    pub fn from_vorticies(vorticies: Vec<Vec<Vortex>>) -> Self {
        let shader_program = ShaderProgram::new(
            "Boundary Vortex",
            "resources/shaders/boundary_vortex.vert",
            "resources/shaders/boundary_vortex.frag",
            HashMap::new(),
        );
        let compute_program_error = ComputeShaderProgram::new(
            "resources/shaders/boundary_vortex_error.comp",
            HashMap::from([(
                "get_velocity",
                fs::read_to_string("resources/gpu_methods/get_velocity.glsl")
                    .unwrap()
                    .as_str(),
            )]),
        );
        let matricies = BoundaryVorticies::create_matricies_from_vorticies(&vorticies);

        let compute_program_correction = ComputeShaderProgram::new(
            "resources/shaders/boundary_vortex_correction.comp",
            HashMap::new(),
        );
        let ssbo_vorticies = util::create_buffer();
        BoundaryVorticies::load_vorticies_to_ssbo(ssbo_vorticies, &vorticies);
        let ssbo_matrix = util::create_buffer();
        BoundaryVorticies::load_matrix_to_ssbo(ssbo_matrix, &matricies);

        let ssbo_errors = util::create_buffer();
        BoundaryVorticies::load_errors_to_ssbo(ssbo_errors, vorticies[0].len());

        let vao = util::create_vao();
        let geometry = Geometry::from_gltf("resources/models/arrow.glb", vao);
        let stepper = BoundaryInfoStepper::new(matricies);

        unsafe {
            let texture = Texture::new();
            texture
                .load(Path::new("resources/textures/kor.png"))
                .unwrap();

            BoundaryVorticies {
                shader_program,
                geometry,
                compute_program_error,
                compute_program_correction,
                vorticies,
                ssbo_errors,
                active_index: 0,
                stepper,
                texture,
            }
        }
    }

    pub fn create_vorticies_from_folder_path(folder_path: &str) -> Vec<Vec<Vortex>> {
        std::fs::read_dir(folder_path)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let path = path.to_str().unwrap();
                let positions = util::get_vertices_and_normals_from_gltf(path);
                BoundaryVorticies::vorticies_from_positions(positions)
            })
            .collect::<Vec<_>>()
    }

    pub fn create_matricies_from_vorticies(vorticies: &Vec<Vec<Vortex>>) -> Vec<DMatrix<f32>> {
        vorticies
            .iter()
            .map(|vorticies| {
                let matrix = BoundaryVorticies::create_matrix(vorticies);
                let current_time = std::time::Instant::now();
                let matrix: DMatrix<f32> = matrix.pseudo_inverse(0.0001f32).unwrap().transpose();
                // .purify();
                println!("Inversion time: {}", current_time.elapsed().as_secs_f32());
                matrix
            })
            .collect::<Vec<DMatrix<f32>>>()
    }

    pub fn vorticies_from_positions(
        positions_and_normals: Vec<(Vector3<f32>, Vector3<f32>)>,
    ) -> Vec<Vortex> {
        positions_and_normals
            .iter()
            .map(|(position, normal)| Vortex {
                position: position.extend(1.),
                vorticity: (normal.normalize() / 10.).extend(1.),
                ..Default::default()
            })
            .collect::<Vec<Vortex>>()
    }

    fn load_vorticies_to_ssbo(ssbo: u32, vorticies: &[Vec<Vortex>]) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            let vorticies = vorticies.iter().flatten().cloned().collect::<Vec<_>>();
            let size = vorticies.len() * std::mem::size_of::<Vortex>();
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                size as isize,
                vorticies.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 10, ssbo);
        }
    }

    fn load_matrix_to_ssbo(ssbo: u32, matrix: &[DMatrix<f32>]) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            let matrix = matrix.iter().flatten().cloned().collect::<Vec<_>>();
            let size = matrix.len() * std::mem::size_of::<f32>();
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                size as isize,
                matrix.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 20, ssbo);
        }
    }

    fn load_errors_to_ssbo(ssbo: u32, vorticies_length: usize) {
        let errors = vec![Vector4::<f32>::zero(); vorticies_length];
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (errors.len() * std::mem::size_of::<Vector4<f32>>()) as isize,
                errors.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 30, ssbo);
        }
    }

    pub fn create_matrix(vorticies: &Vec<Vortex>) -> DMatrix<f32> {
        let length = vorticies.len();
        let mut effect_matrix = DMatrix::<f32>::zeros(length * 3, length * 3);
        for ((ai, a), (bi, b)) in vorticies
            .iter()
            .enumerate()
            .cartesian_product(vorticies.iter().enumerate())
        {
            // if ai == bi {
            //     continue;
            // }
            for (i, j) in (0..3).cartesian_product(0..3) {
                if i == j {
                    continue;
                }
                let mut unit = Vector3::zero();
                unit[i] = 1.0f32;
                effect_matrix[(ai * 3 + i, bi * 3 + j)] = BoundaryVorticies::get_velocity(
                    &a,
                    &Vortex {
                        vorticity: unit.extend(0.0f32),
                        ..*b
                    },
                )[j];
            }
        }
        effect_matrix
    }

    pub fn create_error_vector(
        vorticies: &[Vortex],
        active_vorticies: &[Vortex],
        should_check_boundary: bool,
    ) -> Vec<Vector3<f32>> {
        let mut error: Vec<Vector3<f32>> = vec![];
        for a in vorticies.iter() {
            let mut sum = Vector3::new(0.0f32, 0.0f32, 0.0f32);
            if should_check_boundary {
                for b in vorticies.iter() {
                    sum += BoundaryVorticies::get_velocity(a, b);
                }
            }

            for b in active_vorticies {
                sum += BoundaryVorticies::get_velocity(a, b);
            }
            error.push(sum);
        }
        error
    }

    pub fn calculate_corrections(
        matrix: &DMatrix<f32>,
        errors: &Vec<Vector3<f32>>,
    ) -> Vec<Vector3<f32>> {
        let errors = errors.clone();

        let errors_vector = DMatrix::from_iterator(
            errors.len() * 3,
            1,
            errors.iter().cloned().flat_map(|x| vec![x.x, x.y, x.z]),
        );

        let corrections_f = matrix.transpose() * errors_vector;

        let mut corrections = vec![];

        for i in 0..errors.len() {
            corrections.push(Vector3::new(
                corrections_f[(i * 3, 0)],
                corrections_f[(i * 3 + 1, 0)],
                corrections_f[(i * 3 + 2, 0)],
            ));
        }

        corrections
    }

    fn get_velocity(a: &Vortex, b: &Vortex) -> Vector3<f32> {
        let diff = (b.position - a.position).truncate();
        let distance = diff.magnitude();
        if distance < 0.0001f32 {
            return Vector3::zero();
        }
        b.vorticity.truncate().cross(diff / (distance.pow(3)))
    }

    fn current_length(&self) -> usize {
        self.vorticies[self.active_index].len()
    }

    pub fn get_errors(&self) -> Vec<Vector3<f32>> {
        let mut errors = vec![Vector4::<f32>::zero(); self.current_length()];
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.ssbo_errors);
            gl::GetBufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                0,
                (errors.len() * std::mem::size_of::<Vector4<f32>>()) as isize,
                errors.as_mut_ptr().cast(),
            );
        }
        errors.iter().map(|v| v.truncate()).collect::<Vec<_>>()
    }
}

impl Drawable for BoundaryVorticies {
    fn draw(&self, camera: &PerspectiveCamera) {
        self.shader_program.use_program();
        unsafe {
            self.shader_program
                .bind_uniform_matrix4fv("viewProjectionMatrix", &camera.view_proj_matrix[0][0]);
            self.texture.bind();
            self.shader_program.bind_uniform_4fv(
                "color",
                vec![Vector4 {
                    x: 0f32,
                    y: 1f32,
                    z: 0f32,
                    w: 1f32,
                }],
            );
            self.geometry.draw_instanced(self.current_length() as i32);
        };
    }
}

impl Steppable for BoundaryVorticies {
    fn step(&mut self, dt: f32, camera: &PerspectiveCamera) {
        self.step_errors(dt);
        self.step_correction(dt);
        // let stats = crate::support::magnitude_statistics::MagnitudeStatistics::from_vectors(
        //     &self.get_errors(),
        // );
        // println!("Stats: {:?}", stats);
        self.stepper.step(dt, camera);
    }
}

impl BoundaryVorticies {
    pub fn step_errors(&mut self, dt: f32) {
        unsafe {
            self.compute_program_error.use_program();

            gl::Uniform1f(0, dt);

            gl::DispatchCompute(self.current_length() as u32, 1, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }

    pub fn step_correction(&mut self, dt: f32) {
        unsafe {
            self.compute_program_correction.use_program();

            gl::Uniform1f(0, dt);

            gl::DispatchCompute(self.current_length() as u32, 1, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }
}
