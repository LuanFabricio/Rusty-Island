use glium::Surface;

pub struct Mesh {
    vertices: glium::VertexBuffer<super::vertex::Vertex>,
    indices: glium::index::NoIndices,
    shader_program: glium::Program,
}

impl Mesh {
    pub fn new(
        vertices: glium::VertexBuffer<super::vertex::Vertex>,
        indices: glium::index::NoIndices,
        shader_program: glium::Program,
    ) -> Self {
        Self {
            vertices,
            indices,
            shader_program,
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        let result = frame.draw(
            &self.vertices,
            &self.indices,
            &self.shader_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        );

        if let Some(err) = result.err() {
            println!("Erro! {err}");
        }
    }
}
