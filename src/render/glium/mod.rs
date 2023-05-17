mod mesh;
mod vertex;

use glium::Surface;

use crate::traits::Draw;

use self::{mesh::Mesh, vertex::Vertex};

pub struct GliumRender {
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: glium::Display,
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
        }
    }

    fn draw_scene<const W: usize, const H: usize>(&mut self, height_map: [[f32; H]; W]) {
        let triangle_shape = vec![
            Vertex {
                position: [0_f32, 0_f32, 0_f32],
            },
            Vertex {
                position: [0.5_f32, 0.5_f32, 0_f32],
            },
            Vertex {
                position: [0_f32, 0.5_f32, 0_f32],
            },
        ];

        let triangle_vertex_buffer =
            glium::VertexBuffer::new(&self.display, &triangle_shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140

            in vec3 position;

            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let shader_program = glium::Program::from_source(
            &self.display,
            vertex_shader_src,
            fragment_shader_src,
            None,
        )
        .unwrap();

        let triangle_mesh = Mesh::new(triangle_vertex_buffer, indices, shader_program);

        let mut frame = self.display.draw();

        frame.clear_color(0_f32, 0_f32, 0_f32, 1_f32);

        triangle_mesh.draw(&mut frame);

        frame.finish().unwrap();
    }
}
