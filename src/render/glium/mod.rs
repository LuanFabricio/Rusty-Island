pub mod camera;
pub mod mesh;
pub mod normal;
pub mod util;
pub mod vertex;

use glium::Surface;

use self::{camera::Camera, mesh::Mesh, vertex::Vertex};

pub struct GliumRender {
    pub display: glium::Display,
    camera: Camera,
    meshes: Vec<Mesh>,
    light: [f32; 3],
}

impl GliumRender {
    /// Constructor to create the glium render.
    ///
    /// # Arguments
    /// * `title` - Title of the window.
    ///
    pub fn new(
        title: &str,
        camera_pos: [f32; 3],
    ) -> (Self, glium::glutin::event_loop::EventLoop<()>) {
        let event_loop = glium::glutin::event_loop::EventLoop::new();

        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
            .with_title(title);

        let cb = glium::glutin::ContextBuilder::new();

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        (
            Self {
                display,
                camera: Camera::new(camera_pos, [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
                meshes: vec![],
                light: [10_f32, 15_f32, -10_f32],
            },
            event_loop,
        )
    }

    /// Function to draw the scene (using the meshes of the struct).
    pub fn draw_scene<const W: usize, const H: usize>(
        &mut self,
        scene: &mut crate::scene::Scene<W, H>,
    ) {
        let mut frame = self.display.draw();

        frame.clear_color_and_depth((1_f32, 1_f32, 1_f32, 1_f32), 1.0);

        let perspective = self.get_perspective_matrix(&frame);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let default_uniforms = (self.camera.get_view_matrix(), perspective, self.light);

        scene.draw_entities(&mut frame, default_uniforms, &params);

        for mesh in self.meshes.iter() {
            mesh.draw(
                &mut frame,
                &glium::uniform! {
                    view: self.camera.get_view_matrix(),
                    perspective: perspective,
                    u_light: self.light,
                    matrix: mesh.matrix,
                    ambient_color: mesh.ambient,
                    diffuse_color: mesh.diffuse,
                    specular_color: mesh.specular,
                },
                &params,
            );
        }

        frame.finish().unwrap();
    }
}

impl GliumRender {
    /// Default function to create a simple vertex shader program.
    pub fn create_default_vertex_shader() -> &'static str {
        r#"
            #version 140

            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 matrix;

            void main() {
                mat4 modelview = view * matrix;
                v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = perspective * modelview * vec4(position, 1.0);
            }
        "#
    }

    /// Default function to create a simple fragment shader program.
    pub fn create_default_fragment_shader() -> &'static str {
        r#"
            #version 140

            in vec3 v_normal;
            in vec3 v_position;

            out vec4 color;

            uniform vec3 u_light;

            uniform vec3 ambient_color;
            uniform vec3 diffuse_color;
            uniform vec3 specular_color;

            void main() {
                float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

                vec3 camera_dir = normalize(-v_position);
                vec3 half_direction = normalize(normalize(u_light) + camera_dir);
                float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

                color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
            }
        "#
    }

    /// Function to calculate the perspective matrix (for the scene).
    ///
    /// # Arguments
    /// * `frame` - Frame that will be used to draw.
    ///
    fn get_perspective_matrix(&self, frame: &glium::Frame) -> [[f32; 4]; 4] {
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = self.camera.get_fov().to_radians();
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    /// Function to add the camera position.
    ///
    /// # Arguments
    /// * `vec_pos` - Vector to move the camera (adding).
    ///
    pub fn walk(&mut self, direction: camera::WalkDirection) {
        self.camera.walk(direction);
    }

    pub fn rotate_camera(&mut self, angles: (f32, f32)) {
        self.camera.rotate(angles);
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn zoom_in(&mut self) {
        self.camera.zoom_in();
    }

    pub fn zoom_out(&mut self) {
        self.camera.zoom_out();
    }
}
