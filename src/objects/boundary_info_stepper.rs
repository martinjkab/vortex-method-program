use std::collections::HashMap;

use nalgebra::DMatrix;

use crate::{
    compute_shader_program::ComputeShaderProgram,
    gl::{self},
    structures::vortex::BoundaryInfo,
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
    util::{self},
};

pub struct BoundaryInfoStepper {
    compute_program: ComputeShaderProgram,
}

impl BoundaryInfoStepper {
    pub fn new(matricies: Vec<DMatrix<f32>>) -> BoundaryInfoStepper {
        let compute_program = ComputeShaderProgram::new(
            "resources/shaders/boundary_info_stepper.comp",
            HashMap::new(),
        );
        let ssbo = util::create_buffer();
        BoundaryInfoStepper::load_info_to_ssbo(
            ssbo,
            BoundaryInfo {
                active_index: 0,
                active_count: matricies.first().unwrap().column_iter().len() as i32,
                count: matricies.len() as i32,
            },
        );

        BoundaryInfoStepper { compute_program }
    }

    fn load_info_to_ssbo(ssbo: u32, info: BoundaryInfo) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            let size = std::mem::size_of::<BoundaryInfo>();
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                size as isize,
                &info as *const BoundaryInfo as *const _,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 50, ssbo);
        }
    }
}

impl Drawable for BoundaryInfoStepper {
    fn draw(&self, _camera: &PerspectiveCamera) {}
}

impl Steppable for BoundaryInfoStepper {
    fn step(&mut self, _dt: f32, _camera: &PerspectiveCamera) {
        self.compute_program.use_program();
        unsafe {
            gl::DispatchCompute(1, 1, 1);
            gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
        }
    }
}
