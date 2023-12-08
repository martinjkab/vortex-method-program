use cgmath::{
    EuclideanSpace, InnerSpace, Matrix4, PerspectiveFov, Point3, Rad, SquareMatrix, Transform,
    Vector2, Vector3,
};
use glfw::WindowEvent;

use crate::structures::ray::Ray;

pub struct PerspectiveCamera {
    pub position: Point3<f32>,
    pub roll: Rad<f32>,
    pub pitch: Rad<f32>,
    pub yaw: Rad<f32>,

    pub aspect: f32,

    pub near_plane: f32,
    pub far_plane: f32,

    pub ahead: Vector3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,

    pub speed: f32,
    pub is_dragging: bool,
    pub mouse_delta: Vector2<f32>,

    pub width: u32,
    pub height: u32,

    pub mouse_screen_position: Vector2<f32>,

    pub rotation_matrix: Matrix4<f32>,
    pub view_proj_matrix: Matrix4<f32>,
    pub ray_dir_matrix: Matrix4<f32>,

    pub billboard_size: Vector2<f32>,

    pub world_up: Vector3<f32>,

    moving_forward: bool,
    moving_backward: bool,
    moving_left: bool,
    moving_right: bool,

    last_mouse_position: Vector2<f32>,
}

impl PerspectiveCamera {
    pub fn new(width: u32, height: u32) -> PerspectiveCamera {
        let mut camera = PerspectiveCamera {
            position: Point3::new(0.0, 0.0, 1.0),
            roll: Rad(0.0),
            pitch: Rad(0.0),
            yaw: Rad(0.0),

            aspect: 1.0,

            near_plane: 0.1,
            far_plane: 1000.0,

            ahead: Vector3::new(0.0, 0.0, -1.0),
            right: Vector3::new(1.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),

            width,
            height,

            speed: 0.1,
            is_dragging: false,
            mouse_delta: Vector2::new(0.0, 0.0),
            mouse_screen_position: Vector2::new(0.0, 0.0),

            rotation_matrix: Matrix4::identity(),
            view_proj_matrix: Matrix4::identity(),
            ray_dir_matrix: Matrix4::identity(),

            billboard_size: Vector2::new(0.1, 0.1),

            world_up: Vector3::new(0.0, 1.0, 0.0),
            moving_forward: false,
            moving_backward: false,
            moving_left: false,
            moving_right: false,

            last_mouse_position: Vector2::new(0.0, 0.0),
        };
        camera.update(0f32);
        camera
    }

    pub fn fov(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn update(&mut self, dt: f32) {
        self.rotation_matrix = Matrix4::from_angle_z(self.roll)
            * Matrix4::from_angle_x(self.pitch)
            * Matrix4::from_angle_y(self.yaw);
        self.view_proj_matrix =
            self.rotation_matrix * Matrix4::from_translation(self.position.to_vec());
        self.view_proj_matrix = self.view_proj_matrix.invert().unwrap();
        self.view_proj_matrix.transpose_self();

        let mut proj_matrix2: Matrix4<f32> = PerspectiveFov {
            fovy: Rad(self.fov()),
            near: self.near_plane,
            far: self.far_plane,
            aspect: self.aspect,
        }
        .into();
        proj_matrix2.transpose_self();
        self.view_proj_matrix = self.view_proj_matrix * proj_matrix2;

        if self.moving_forward {
            self.position.z -= 10f32 * dt;
        }

        if self.moving_backward {
            self.position.z += 10f32 * dt;
        }

        if self.moving_left {
            self.position.x -= 10f32 * dt;
        }

        if self.moving_right {
            self.position.x += 10f32 * dt;
        }

        if self.is_dragging {
            self.yaw -= Rad(self.mouse_delta.x * 0.002f32);
            self.roll -= Rad(self.mouse_delta.y * 0.002f32);

            self.mouse_delta = Vector2 {
                x: 0 as f32,
                y: 0 as f32,
            };
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.update(0f32);
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.mouse_move(event);
        self.mouse_clicked(event);
        self.process_input(event);
    }

    pub fn process_input(&mut self, event: &WindowEvent) {
        let input = match *event {
            WindowEvent::Key(key, _, action, _) => (key, action),
            _ => return,
        };

        match input {
            (glfw::Key::W, glfw::Action::Press) => {
                self.moving_forward = true;
            }
            (glfw::Key::W, glfw::Action::Release) => {
                self.moving_forward = false;
            }
            (glfw::Key::S, glfw::Action::Press) => {
                self.moving_backward = true;
            }
            (glfw::Key::S, glfw::Action::Release) => {
                self.moving_backward = false;
            }
            (glfw::Key::A, glfw::Action::Press) => {
                self.moving_left = true;
            }
            (glfw::Key::A, glfw::Action::Release) => {
                self.moving_left = false;
            }
            (glfw::Key::D, glfw::Action::Press) => {
                self.moving_right = true;
            }
            (glfw::Key::D, glfw::Action::Release) => {
                self.moving_right = false;
            }
            _ => {}
        }
    }

    pub fn mouse_move(&mut self, event: &WindowEvent) {
        let mouse_position = match *event {
            WindowEvent::CursorPos(x, y) => Vector2 {
                x: x as f32,
                y: y as f32,
            },
            _ => {
                return;
            }
        };

        self.mouse_screen_position = mouse_position;

        if !self.is_dragging {
            self.last_mouse_position = mouse_position;
            return;
        }

        let delta = mouse_position - self.last_mouse_position;

        self.last_mouse_position = mouse_position;

        self.mouse_delta += delta;
    }

    pub fn mouse_clicked(&mut self, event: &WindowEvent) {
        let (button, state) = match *event {
            WindowEvent::MouseButton(button, state, _) => (button, state),
            _ => {
                return;
            }
        };

        match (button, state) {
            (glfw::MouseButton::Button2, glfw::Action::Press) => {
                self.is_dragging = true;
            }
            (glfw::MouseButton::Button2, glfw::Action::Release) => {
                self.is_dragging = false;
            }
            _ => {}
        }
    }

    pub fn unproject_ray(&self) -> Ray {
        let screen = self.mouse_screen_position;
        // Convert screen coordinates to normalized device coordinates (NDC)
        let ndc_x = screen.x / self.width as f32 * 2.0 - 1.0;
        let ndc_y = screen.y / self.height as f32 * 2.0 - 1.0;

        // Create the inverse of the combined view-projection matrix
        let inv_view_projection = (self.view_proj_matrix).invert().unwrap();

        // Create a vector in NDC space
        let ndc_near = Point3::new(ndc_x, ndc_y, -1.0);
        let ndc_far = Point3::new(ndc_x, ndc_y, 1.0);

        // Unproject the NDC vector back to world space
        let world_near = inv_view_projection.transform_point(ndc_near);
        let world_far = inv_view_projection.transform_point(ndc_far);

        // Calculate the direction vector from the near point to the far point

        Ray {
            origin: self.position,
            direction: (world_far - world_near).normalize(),
        }
    }
}
