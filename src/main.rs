use cgmath::Vector4;
use glfw::Context;
use log::error;
use structures::vortex::Vortex;

use crate::{
    objects::{
        active_vorticies::ActiveVorticies,
        particles::{self},
    },
    support::camera::PerspectiveCamera,
    traits::{drawable::Drawable, steppable::Steppable},
};

const WIDTH: u32 = 5000;
const HEIGHT: u32 = 5000;
const TITLE: &str = "Hello From OpenGL World!";

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod compute_shader_program;
pub mod extensions;
pub mod geometry;
pub mod objects;
pub mod shader_program;
pub mod structures;
pub mod support;
pub mod test;
pub mod traits;
pub mod util;

fn main() {
    let (mut glfw, mut window, events) = init_gl();

    let mut camera = PerspectiveCamera::new(1000, 1000);

    let boundary_model_path = "resources/models/cube_normal";

    let mut scene = objects::scene::Scene::new();
    // scene.add(
    //     Particles::new(
    //         positions
    //             .iter()
    //             .map(|p| Particle {
    //                 position: Vector4::new(p.x, p.y, p.z, 1.0),
    //                 lifetime: Vector4::new(0.0, 0.0, 0.0, 0.0),
    //                 velocity: Vector4::new(0.0, 0.0, 0.0, 0.0),
    //             })
    //             .collect(),
    //     )
    //     .with_resetting(false),
    // );
    scene.add(
        particles::Particles::new_inside_unit_sphere(10024)
            .with_fading(true)
            .with_resetting(true),
    );
    // scene.add(objects::boundary_vorticies::BoundaryVorticies::new(
    //     boundary_model_path,
    // ));
    // scene.add(
    //     vorticies::Vorticies::new_random(1024, 5.0, 10.0, 0.0, 0.1)
    //         .with_mirroring(1)
    //         .with_fading(false),
    // );
    // scene.add(
    //     ActiveVorticies::new(
    //         vec![
    //             Vortex {
    //                 position: Vector4::new(0.5, 0.0, 0.0, 1.0),
    //                 vorticity: Vector4::new(1.0, 1.0, 1.0, 1.0),
    //                 lifetime: Vector4::new(5., 10., 0.0, 1.0),
    //                 ..Default::default()
    //             },
    //             // Vortex {
    //             //     position: Vector4::new(1.0, 0.0, 0.0, 1.0),
    //             //     vorticity: Vector4::new(1.0, 1.0, 1.0, 1.0),
    //             //     lifetime: Vector4::new(5., 10., 0.0, 1.0),
    //             //     ..Default::default()
    //             // },
    //         ],
    //         1.0,
    //         2.0,
    //         0.5,
    //         1.0,
    //         0,
    //     )
    //     .with_fading(false),
    // );
    scene.add(ActiveVorticies::new_random(30, 1.0, 3.0, 0.1, 0.2, 0));

    // scene.add(cube_geometry::CubeGeometry::new());
    // scene.add(objects::test_sphere::TestSphere::new(
    //     "resources/models/cube/boundary_cube_2.glb",
    // ));

    // scene.add(objects::boundary_mesh::BoundaryMesh::new(
    //     "resources/models/monkey/monkey_0_n.glb",
    // ));

    let min_dt = 1.0 / 60.0f32;

    let mut last_time = std::time::Instant::now();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, &event);
            camera.handle_window_event(&event);
        }

        let current_time = std::time::Instant::now();

        let dt = (current_time - last_time).as_secs_f32();
        last_time = current_time;
        let mut step = 0.0f32;

        while step < dt {
            let current_step = min_dt.min(dt - step);
            camera.update(current_step);
            scene.step(current_step, &camera);
            step += current_step;
        }

        unsafe {
            gl::ClearColor(0., 0., 0., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        scene.draw(&camera);

        let size = window.get_size();

        camera.set_aspect_ratio(size.0 as f32 / size.1 as f32);
        window.swap_buffers();
    }
}

fn init_gl() -> (
    glfw::Glfw,
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
) {
    let mut glfw = glfw::init(error_callback).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::DepthBits(Some(24)));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .unwrap();
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
    }

    // println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    // println!(
    //     "GLSL version: {}",
    //     gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    // );
    (glfw, window, events)
}

fn error_callback(err: glfw::Error, description: String) {
    error!("GLFW error {:?}: {:?}", err, description);
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn glfw_handle_event(window: &mut glfw::Window, event: &glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    if let Event::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true);
    }
}
