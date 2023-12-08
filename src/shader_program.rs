use std::{collections::HashMap, fs};

use cgmath::Vector4;

use crate::{gl, support::file::Parametrized};

#[derive(Debug, Clone, Default)]
pub struct ShaderProgram {
    pub location: u32,
    pub name: String,
}

impl ShaderProgram {
    pub fn new(
        name: &str,
        vertex_source: &str,
        fragment_source: &str,
        params: HashMap<&str, &str>,
    ) -> ShaderProgram {
        let shader_program = ShaderProgram::create_shader_program();
        let vertex_shader = ShaderProgram::compile_vertex_shader(vertex_source, params.clone());
        let fragment_shader = ShaderProgram::compile_fragment_shader(fragment_source, params);
        let shader_program =
            ShaderProgram::link_shader_program(shader_program, vertex_shader, fragment_shader);
        let name = name.to_string();

        ShaderProgram {
            location: shader_program,
            name,
        }
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

    fn link_shader_program(shader_program: u32, vertex_shader: u32, fragment_shader: u32) -> u32 {
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
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
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::UseProgram(shader_program);

            shader_program
        }
    }

    fn compile_vertex_shader(vertex_source: &str, params: HashMap<&str, &str>) -> u32 {
        let vert_shader = fs::read_to_string(vertex_source)
            .unwrap()
            .parametrize(params);

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vert_shader.as_bytes().as_ptr().cast()),
                &(vert_shader.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex_shader);
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            }
            let mut info_log_length = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
            if info_log_length > 0 {
                let mut vertex_shader_error_message: Vec<u8> =
                    vec![0; (info_log_length + 1) as usize];
                gl::GetShaderInfoLog(
                    vertex_shader,
                    info_log_length,
                    &mut 10000,
                    vertex_shader_error_message.as_mut_ptr().cast(),
                );
                println!("Vertex Info Log: {:?}", &vertex_shader_error_message[0]);
            }
            vertex_shader
        }
    }

    fn compile_fragment_shader(fragment_source: &str, params: HashMap<&str, &str>) -> u32 {
        let frag_shader = fs::read_to_string(fragment_source)
            .unwrap()
            .parametrize(params);

        unsafe {
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(frag_shader.as_bytes().as_ptr().cast()),
                &(frag_shader.len().try_into().unwrap()),
            );
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }
            fragment_shader
        }
    }

    fn get_location(&self, name: &str) -> i32 {
        unsafe {
            let name = format!("{name}\0");
            let p = name.as_ptr();
            let location = gl::GetUniformLocation(self.location, p.cast());
            assert!(location >= 0);
            location
        }
    }

    pub fn bind_uniform_1f(&self, name: &str, data: f32) {
        let location = self.get_location(name);
        unsafe {
            if location >= 0 {
                gl::Uniform1f(location, data);
            }
        }
    }

    pub fn bind_uniform_1ui(&self, name: &str, data: u32) {
        let location = self.get_location(name);
        unsafe {
            if location >= 0 {
                gl::Uniform1ui(location, data);
            }
        }
    }

    pub fn bind_uniform_4fv(&self, name: &str, data: Vec<Vector4<f32>>) {
        let location = self.get_location(name);
        unsafe {
            if location >= 0 {
                gl::Uniform4fv(location, 1, data.as_ptr().cast());
            }
        }
    }

    /// .
    ///
    /// # Safety
    ///
    /// .
    pub unsafe fn bind_uniform_matrix4fv(&self, name: &str, data: *const f32) {
        let location = self.get_location(name);
        unsafe {
            if location >= 0 {
                gl::UniformMatrix4fv(location, 1, false as u8, data);
            }
        }
    }
}
