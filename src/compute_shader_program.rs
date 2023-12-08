use std::{collections::HashMap, fs};

use crate::{gl, support::file::Parametrized};

#[derive(Debug, Clone, Copy, Default)]
pub struct ComputeShaderProgram {
    pub location: u32,
}

impl ComputeShaderProgram {
    pub fn new(compute_source: &str, params: HashMap<&str, &str>) -> ComputeShaderProgram {
        let shader_program = ComputeShaderProgram::create_shader_program();
        let compute_shader = ComputeShaderProgram::compile_compute_shader(compute_source, params);
        let location = ComputeShaderProgram::link_shader_program(shader_program, compute_shader);
        ComputeShaderProgram { location }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.location);
        }
    }

    fn create_shader_program() -> u32 {
        let shader_program = unsafe { gl::CreateProgram() };
        assert_ne!(shader_program, 0);
        shader_program
    }

    fn link_shader_program(shader_program: u32, compute_shader: u32) -> u32 {
        unsafe {
            gl::AttachShader(shader_program, compute_shader);
            gl::LinkProgram(shader_program);
            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::DeleteShader(compute_shader);
            gl::UseProgram(shader_program);

            shader_program
        }
    }

    fn compile_compute_shader(compute_source: &str, params: HashMap<&str, &str>) -> u32 {
        let comp_shader = fs::read_to_string(compute_source)
            .unwrap()
            .parametrize(params);

        unsafe {
            let compute_shader = gl::CreateShader(gl::COMPUTE_SHADER);
            assert_ne!(compute_shader, 0);
            gl::ShaderSource(
                compute_shader,
                1,
                &(comp_shader.as_bytes().as_ptr().cast()),
                &(comp_shader.len().try_into().unwrap()),
            );
            gl::CompileShader(compute_shader);
            let mut success = 0;
            gl::GetShaderiv(compute_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(compute_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Compute Compile Error: {}", String::from_utf8_lossy(&v));
            }
            compute_shader
        }
    }

    pub fn bind_uniform_1f(&self, name: &str, data: f32) {
        unsafe {
            let location = self.get_location(name);
            if location >= 0 {
                gl::Uniform1f(location, data);
            }
        }
    }

    fn get_location(&self, name: &str) -> i32 {
        unsafe { gl::GetUniformLocation(self.location, name.as_ptr() as *const gl::types::GLchar) }
    }

    pub fn resource_location(&self, name: &str) -> u32 {
        unsafe {
            gl::GetProgramResourceIndex(
                self.location,
                gl::SHADER_STORAGE_BLOCK,
                name.as_ptr().cast(),
            )
        }
    }
}
