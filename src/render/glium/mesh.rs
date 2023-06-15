use glium::Surface;

use super::{util::rotate_y_matrix, GliumRender};

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

    pub fn from_obj(obj: &obj::Obj, display: &glium::Display) -> Self {
        let mut vertices: Vec<super::Vertex> = vec![];
        for obj_vertex in obj.vertices.iter() {
            vertices.push(super::Vertex {
                position: obj_vertex.position,
                normal: obj_vertex.normal,
            })
        }
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();

        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &obj.indices,
        )
        .unwrap();

        let shader_program = glium::Program::from_source(
            display,
            GliumRender::create_default_vertex_shader(),
            GliumRender::create_default_fragment_shader(),
            None,
        )
        .unwrap();

        Self {
            vertices: vertex_buffer,
            indices: index_buffer,
            shader_program,
            ambient: [0_f32; 3],
            diffuse: [1_f32, 1_f32, 1_f32],
            specular: [0_f32, 0_f32, 0.5_f32],
            matrix: [
                [1_f32, 0_f32, 0_f32, 0_f32],
                [0_f32, 1_f32, 0_f32, 0_f32],
                [0_f32, 0_f32, 1_f32, 0_f32],
                [0_f32, 0_f32, 0_f32, 1_f32],
            ],
        }
    }

    pub fn set_diffuse(&mut self, diffuse: [f32; 3]) {
        self.diffuse = diffuse;
    }

    pub fn set_specular(&mut self, specular: [f32; 3]) {
        self.specular = specular;
    }

    pub fn set_position(&mut self, new_position: [f32; 3]) {
        self.matrix[3][0] = new_position[0];
        self.matrix[3][1] = new_position[1];
        self.matrix[3][2] = new_position[2];
    }

    pub fn set_rotation_y(&mut self, angle: f32) {
        rotate_y_matrix(&mut self.matrix, angle);
    }
}
