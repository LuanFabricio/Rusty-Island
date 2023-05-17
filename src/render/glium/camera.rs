pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3],
}

impl Camera {
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> Self {
        Self {
            position,
            direction,
            up,
        }
    }

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
}
