use glium::Surface;

pub struct Mesh {
    vertices: glium::VertexBuffer<super::vertex::Vertex>,
    normals: glium::VertexBuffer<super::normal::Normal>,
    indices: glium::IndexBuffer<u16>,
    shader_program: glium::Program,
    pub matrix: [[f32; 4]; 4],
}

impl Mesh {
    pub fn new(
        vertices: glium::VertexBuffer<super::vertex::Vertex>,
        normals: glium::VertexBuffer<super::normal::Normal>,
        indices: glium::IndexBuffer<u16>,
        shader_program: glium::Program,
        matrix: [[f32; 4]; 4],
    ) -> Self {
        Self {
            vertices,
            normals,
            indices,
            shader_program,
            matrix,
        }
    }

    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        uniforms: &impl glium::uniforms::Uniforms,
        params: &glium::DrawParameters,
    ) {
        let result = frame.draw(
            (&self.vertices, &self.normals),
            &self.indices,
            &self.shader_program,
            uniforms,
            params,
        );

        if let Some(err) = result.err() {
            println!("Erro! {err}");
        }
    }
}
