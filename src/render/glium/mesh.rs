use glium::Surface;

pub struct Mesh {
    vertices: glium::VertexBuffer<super::vertex::Vertex>,
    indices: glium::IndexBuffer<u16>,
    shader_program: glium::Program,
    pub matrix: [[f32; 4]; 4],
    pub specular: [f32; 3],
    pub diffuse: [f32; 3],
    pub ambient: [f32; 3],
}

impl Mesh {
    /// Constructor for the struct.
    ///
    /// # Arguments
    /// * `vertices` - Buffer with vertices data.
    /// * `indices` - Indices for render the mesh.
    /// * `shader_program` - Shaders for the mesh.
    /// * `matrix` - Matrix with the mesh data(position, rotation and scale).
    /// * `colors` - Colors for the mesh (specular, diffuse, ambient).
    ///
    pub fn new(
        vertices: glium::VertexBuffer<super::vertex::Vertex>,
        indices: glium::IndexBuffer<u16>,
        shader_program: glium::Program,
        matrix: [[f32; 4]; 4],
        colors: ([f32; 3], [f32; 3], [f32; 3]),
    ) -> Self {
        Self {
            vertices,
            indices,
            shader_program,
            matrix,
            specular: colors.0,
            diffuse: colors.1,
            ambient: colors.2,
        }
    }

    /// Function to draw mesh on scene.
    ///
    /// # Arguments
    /// * `frame` - Frame where we draw.
    /// * `uniforms` - Uniforms for shaders.
    /// * `params` - Params with the scene configuration.
    ///
    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        uniforms: &impl glium::uniforms::Uniforms,
        params: &glium::DrawParameters,
    ) {
        let result = frame.draw(
            &self.vertices,
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
