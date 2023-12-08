use std::mem::size_of;

use cgmath::Vector4;
use easy_gltf::load;

use crate::gl;

pub struct Geometry {
    vao: u32,
    ibo: u32,
    index_length: usize,
}

impl Geometry {
    pub fn new(vao: u32, vertices: Vec<Vector4<f32>>, indices: Vec<u32>) -> Geometry {
        let vbo = Geometry::create_buffer();
        let ibo = Geometry::create_buffer();
        Geometry::load_data_to_vbo(vao, vbo, vertices);
        Geometry::load_data_to_ibo(vao, ibo, indices.clone());
        Geometry {
            vao,
            ibo,
            index_length: indices.len(),
        }
    }

    pub fn new_square(vao: u32) -> Geometry {
        let vertices: Vec<Vector4<f32>> = vec![
            Vector4::new(-1., -1., 0.999999f32, 1.),
            Vector4::new(-1., 1., 0.999999f32, 1.),
            Vector4::new(1., -1., 0.999999f32, 1.),
            Vector4::new(1., 1., 0.999999f32, 1.),
        ];

        let indices = vec![0, 1, 2, 1, 2, 3];

        Geometry::new(vao, vertices, indices)
    }

    fn create_buffer() -> u32 {
        unsafe {
            let mut vao = 0;
            gl::GenBuffers(1, &mut vao);
            assert_ne!(vao, 0);

            vao
        }
    }

    fn load_data_to_vbo(vao: u32, vbo: u32, vertices: Vec<Vector4<f32>>) {
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vector4<f32>>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (size_of::<Vector4<f32>>()).try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    fn load_data_to_ibo(vao: u32, ibo: u32, indices: Vec<u32>) {
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_length as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        };
    }

    pub fn draw_instanced(&self, count: i32) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.index_length as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                count,
            );
            gl::BindVertexArray(0);
        };
    }

    pub fn from_gltf(model_path: &str, vao: u32) -> Self {
        let scenes = load(model_path).expect("Failed to load gltf file");
        let first_scene = scenes.into_iter().next().expect("No scenes in gltf file");
        let vertices = first_scene
            .models
            .iter()
            .flat_map(|m| m.vertices().clone())
            .map(|v| {
                let v = v.position;
                v.extend(1.)
            })
            .collect::<Vec<_>>();
        let indices = first_scene
            .models
            .iter()
            .filter_map(|m| m.indices())
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        Geometry::new(vao, vertices, indices)
    }
}
