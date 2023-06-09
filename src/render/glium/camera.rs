use super::util::{cross_vec3, normalize};

const ZOOM_SPEED: f32 = 1.5f32;
const MAX_FOV: f32 = 90f32;
const INITIAL_FOV: f32 = 90f32;
const MIN_FOV: f32 = 0_f32;

static mut DEFAULT_DIRECTION: [f32; 3] = [0_f32, -1_f32, 0_f32];
static mut DEFAULT_RIGHT: [f32; 3] = [0_f32, 0_f32, 1_f32];
static mut DEFAULT_UP: [f32; 3] = [-1_f32, 0_f32, 1_f32];

pub enum WalkDirection {
    Front,
    Back,
    Left,
    Right,
    Up,
    Down,
}

pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    right: [f32; 3],
    up: [f32; 3],
    rotation_x: f32,
    rotation_y: f32,
    fov: f32,
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
        unsafe {
            DEFAULT_DIRECTION = direction;
            DEFAULT_UP = up;
            DEFAULT_RIGHT = cross_vec3(direction, up);

            Self {
                position,
                direction,
                right: DEFAULT_RIGHT,
                up,
                rotation_x: 0_f32,
                rotation_y: 0_f32,
                fov: INITIAL_FOV,
            }
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

    pub fn walk(&mut self, direction: WalkDirection) {
        let vec_pos = match direction {
            WalkDirection::Front => self.direction,
            WalkDirection::Back => [-self.direction[0], -self.direction[1], -self.direction[2]],
            WalkDirection::Left => [-self.right[0], -self.right[1], -self.right[2]],
            WalkDirection::Right => self.right,
            WalkDirection::Up => self.up,
            WalkDirection::Down => [-self.up[0], -self.up[1], -self.up[2]],
        };

        self.add_position(vec_pos);
    }

    pub fn rotate(&mut self, rotation: (f32, f32)) {
        self.rotation_x = self.rotation_x + rotation.0;
        self.rotation_y = self.rotation_y + rotation.1;

        self.rotate_right();
        self.rotate_up();
        self.rotate_direction();
    }

    fn rotate_up(&mut self) {
        let radians = self.rotation_y.to_radians();

        unsafe {
            self.up[1] = DEFAULT_UP[1] * radians.cos() - DEFAULT_UP[2] * radians.sin();
            self.up[2] = DEFAULT_UP[1] * radians.sin() + DEFAULT_UP[2] * radians.cos();
        }

        self.up = normalize(self.up);
    }

    fn rotate_right(&mut self) {
        let radians = self.rotation_x.to_radians();

        unsafe {
            self.right[0] = DEFAULT_RIGHT[0] * radians.cos() - DEFAULT_RIGHT[1] * radians.sin();
            self.right[1] = -DEFAULT_RIGHT[0] * radians.sin() + DEFAULT_RIGHT[1] * radians.cos();
        }

        self.right = normalize(self.right);
    }

    fn rotate_direction(&mut self) {
        self.direction = normalize(cross_vec3(self.up, self.right));
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn zoom_in(&mut self) {
        self.fov = (self.fov - ZOOM_SPEED).max(MIN_FOV);
    }

    pub fn zoom_out(&mut self) {
        self.fov = (self.fov + ZOOM_SPEED).min(MAX_FOV);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_camera() -> Camera {
        Camera::new([0_f32; 3], [0_f32, 0_f32, 1_f32], [0_f32, 1_f32, 0_f32])
    }

    mod get_fov {
        use super::*;

        #[test]
        fn should_return_current_fov() {
            let mut camera = create_camera();

            assert_eq!(camera.get_fov(), INITIAL_FOV);

            camera.zoom_in();
            assert_eq!(camera.get_fov(), INITIAL_FOV - ZOOM_SPEED);

            camera.zoom_out();
            assert_eq!(camera.get_fov(), INITIAL_FOV);
        }
    }

    mod zoom_in {
        use super::*;

        #[test]
        fn should_be_greater_or_equal_than_max_fov() {
            let mut camera = create_camera();

            assert_eq!(camera.fov, INITIAL_FOV);

            camera.fov = 0_f32;
            camera.zoom_in();

            assert!(camera.fov >= MIN_FOV);

            camera.fov = MIN_FOV;
            camera.zoom_in();

            assert_eq!(camera.fov, MIN_FOV);
        }
    }

    mod zoom_out {
        use super::*;

        #[test]
        fn should_be_less_or_equal_than_max_fov() {
            let mut camera = create_camera();

            assert_eq!(camera.fov, INITIAL_FOV);

            camera.fov = 90_f32;
            camera.zoom_out();

            assert!(camera.fov <= MAX_FOV);

            camera.fov = MAX_FOV;
            camera.zoom_out();

            assert_eq!(camera.fov, MAX_FOV);
        }
    }
}
