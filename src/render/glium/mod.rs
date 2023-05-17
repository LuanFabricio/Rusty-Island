mod camera;
mod mesh;
mod normal;
mod teapot;
mod util;
mod vertex;

use glium::Surface;

use crate::traits::Draw;

use self::{camera::Camera, mesh::Mesh, normal::Normal, vertex::Vertex};

pub struct GliumRender {
    pub event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: glium::Display,
    camera: Camera,
}

impl Draw for GliumRender {
    fn new(title: &str) -> Self {
        let event_loop = glium::glutin::event_loop::EventLoop::new();

        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
            .with_title(title);

        let cb = glium::glutin::ContextBuilder::new();

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Self {
            event_loop,
            display,
            camera: Camera::new([2.0, -1.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]),
        }
    }

    fn draw_scene<const W: usize, const H: usize>(&mut self, height_map: [[f32; H]; W]) {
        let triangle_shape = self::teapot::VERTICES;

        let triangle_normals = self::teapot::NORMALS;

        let triangle_vertex_buffer =
            glium::VertexBuffer::new(&self.display, &triangle_shape).unwrap();
        let triangle_normals_buffer =
            glium::VertexBuffer::new(&self.display, &triangle_normals).unwrap();

        let indices = glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &self::teapot::INDICES,
        )
        .unwrap();

        let vertex_shader_src = GliumRender::create_default_vertex_shader();

        let fragment_shader_src = GliumRender::create_default_fragment_shader();

        let shader_program = self
            .create_shader_program(vertex_shader_src, fragment_shader_src)
            .unwrap();

        let triangle_mesh = Mesh::new(
            triangle_vertex_buffer,
            triangle_normals_buffer,
            indices,
            shader_program,
            [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.0, 1.0_f32],
            ],
        );

        let mut frame = self.display.draw();

        frame.clear_color_and_depth((0_f32, 0_f32, 0_f32, 1_f32), 1.0);

        let perspective = GliumRender::get_perspective_matrix(&frame);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let light = [-1.0, 0.4, 0.9_f32];

        println!("{:?}", self.camera.get_view_matrix());

        triangle_mesh.draw(
            &mut frame,
            &glium::uniform! {
                view: self.camera.get_view_matrix(),
                matrix: triangle_mesh.matrix,
                perspective: perspective,
                u_light: light,
            },
            &params,
        );

        frame.finish().unwrap();
    }
}

impl GliumRender {
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

    pub fn create_default_fragment_shader() -> &'static str {
        r#"
            #version 140

            in vec3 v_normal;

            out vec4 color;

            uniform vec3 u_light;

            void main() {
                float brightness = dot(normalize(v_normal), normalize(u_light));
                vec3 dark_color = vec3(0.6, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }
        "#
    }

    pub fn create_shader_program(
        &self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<glium::Program, glium::ProgramCreationError> {
        glium::Program::from_source(&self.display, vertex_shader, fragment_shader, None)
    }

    fn get_perspective_matrix(frame: &glium::Frame) -> [[f32; 4]; 4] {
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
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
}
