use super::util::{cross_vec3, normalize};

const DEFAULT_DIRECTION: [f32; 3] = [0_f32, 0_f32, 1_f32];
const DEFAULT_RIGHT: [f32; 3] = [-1_f32, 0_f32, 0_f32];
const DEFAULT_UP: [f32; 3] = [0_f32, 1_f32, 0_f32];

pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    right: [f32; 3],
    up: [f32; 3],
    rotation_x: f32,
    rotation_y: f32,
}

impl Camera {
    /// Constructor for camera struct.
    ///
    /// # Arguments
    /// * `position` - Position for the camera.
    /// * `direction` - Direction where the camera is looking.
    /// * `up` - Up vector for the camera.
    ///
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> Self {
        Self {
            position,
            direction,
            right: cross_vec3(direction, up),
            up,
            rotation_x: 0_f32,
            rotation_y: 0_f32,
        }
    }

    /// Function to get the view matrix (to change the scene).
    ///
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        let direction = super::util::normalize(self.direction);

        let s = [
            self.up[1] * direction[2] - self.up[2] * direction[1],
            self.up[2] * direction[0] - self.up[0] * direction[2],
            self.up[0] * direction[1] - self.up[1] * direction[0],
        ];

        let s_normalized = super::util::normalize(s);

        let u = [
            direction[1] * s_normalized[2] - direction[2] * s_normalized[1],
            direction[2] * s_normalized[0] - direction[0] * s_normalized[2],
            direction[0] * s_normalized[1] - direction[1] * s_normalized[0],
        ];

        let p = [
            -self.position[0] * s_normalized[0]
                - self.position[1] * s_normalized[1]
                - self.position[2] * s_normalized[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * direction[0]
                - self.position[1] * direction[1]
                - self.position[2] * direction[2],
        ];

        [
            [s_normalized[0], u[0], direction[0], 0.0],
            [s_normalized[1], u[1], direction[1], 0.0],
            [s_normalized[2], u[2], direction[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    /// Function to move the camera position.
    ///
    /// # Arguments
    /// * `vec_pos` - Vector to move the camera (by adding).
    ///
    pub fn add_position(&mut self, vec_pos: [f32; 3]) {
        self.position[0] += vec_pos[0];
        self.position[1] += vec_pos[1];
        self.position[2] += vec_pos[2];
    }

    pub fn rotate(&mut self, rotation: (f32, f32)) {
        // FIX: Rotation not working.
        self.rotation_x = self.rotation_x + rotation.0;
        self.rotation_y = self.rotation_y + rotation.1;

        self.rotate_right();
        self.rotate_up();
        self.rotate_direction();
    }

    fn rotate_up(&mut self) {
        let radians = self.rotation_y * std::f32::consts::PI / 180_f32;

        self.up[1] = self.up[1] * radians.cos() - self.up[2] * radians.sin();
        self.up[2] = self.up[2] * radians.sin() + self.up[2] * radians.cos();

        self.up = normalize(self.up);
    }

    fn rotate_right(&mut self) {
        let radians = self.rotation_x * std::f32::consts::PI / 180_f32;

        self.right[0] = self.right[0] * radians.cos() - self.right[1] * radians.sin();
        self.right[1] = -self.right[0] * radians.sin() + self.right[1] * radians.cos();

        self.right = normalize(self.right);
    }

    fn rotate_direction(&mut self) {
        self.direction = normalize(cross_vec3(self.up, self.right));
    }
}
