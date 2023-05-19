use super::{mesh::Mesh, GliumRender};

/// Create a normalized array with size N.
///
/// # Arguments
/// * `vector` - Vector with de original array.
///
/// # Example
/// let array = [2_f32, 0_f32, 0_f32];
///
/// let normalized_array = normalize(array);
///
/// assert_eq!(normalized_array, [1_f32, 0_f32, 0_f32]);
///
pub fn normalize<const N: usize>(vector: [f32; N]) -> [f32; N] {
    let mut len: f32 = 0_f32;

    for item in vector.iter() {
        len += item * item;
    }

    len = len.sqrt();

    let mut new_vector = [0_f32; N];

    for i in 0..N {
        new_vector[i] = vector[i] / len;
    }

    new_vector
}

/// Calculate the mesh for a height map
///
/// # Arguments
/// * `height_map` - Height map to create the mesh.
/// * `mesh_colors` - Colors (specular, diffuse and ambient) for the mesh
/// * `display` - Display struct to create buffers and shader programs.
///
pub fn height_map_to_mesh<const W: usize, const H: usize>(
    height_map: [[f32; H]; W],
    mesh_colors: ([f32; 3], [f32; 3], [f32; 3]),
    display: &glium::Display,
) -> Mesh {
    let mut vertex_array = Vec::<super::Vertex>::new();
    let mut index_array = Vec::<u16>::new();

    for i in 0..W {
        for j in 0..H {
            let pos = [i as f32, j as f32, height_map[i][j]];
            let normal = get_normal(&height_map, i, j);

            vertex_array.push(super::Vertex {
                position: pos,
                normal,
            });
        }
    }

    for j in 0..W - 1 {
        if j % 2 == 0 {
            for i in 0..H {
                index_array.push((i + j * H) as u16);
                index_array.push((i + (j + 1) * H) as u16);
            }
        } else {
            for i in (1..H).rev() {
                index_array.push((i + (j + 1) * H) as u16);
                index_array.push((i - 1 + j * H) as u16);
            }
        }
    }

    let vertex_shader = GliumRender::create_default_vertex_shader();
    let fragment_shader = GliumRender::create_default_fragment_shader();

    let program =
        glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(display, &vertex_array).unwrap();
    // let normal_buffer = glium::VertexBuffer::new(display, &normal_array).unwrap();

    let indices = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TriangleStrip,
        &index_array,
    )
    .unwrap();

    // println!("{indices:?}");

    // println!("Vertex_array: {vertex_array:?}");
    // println!("Normal_array: {normal_array:?}");
    // println!("Index_array: {index_array:?}");

    let matrix = [
        [1_f32, 0.0_f32, 0.0_f32, 0.0_f32],
        [0.0_f32, 1_f32, 0.0_f32, 0.0_f32],
        [0.0_f32, 0.0_f32, 1_f32, 0.0_f32],
        [0.0_f32, 0.0_f32, 5.0_f32, 1.0_f32],
    ];

    Mesh::new(vertex_buffer, indices, program, matrix, mesh_colors)
}

/// Calculate the normal for a vertex based on their neightbors.
///
/// # Arguments
/// * `height_map` - Height map with size W x H.
/// * `point_x` - The point x of the current vertex.
/// * `point_z` - The point y of the current vertex.
///
fn get_normal<const W: usize, const H: usize>(
    height_map: &[[f32; H]; W],
    point_x: usize,
    point_z: usize,
) -> [f32; 3] {
    // HACK: To gets the neightbors vertex to calculate the normal (using v1 X v2).
    // The correct implementation should use the triangle's face
    let (v1, v2) = match (point_x + 1 < W, point_z + 1 < H, point_x > 0, point_z > 0) {
        (true, true, _, _) => (
            [
                point_x as f32,
                height_map[point_x][point_z + 1],
                (point_z + 1) as f32,
            ],
            [
                (point_x + 1) as f32,
                height_map[point_x + 1][point_z],
                point_z as f32,
            ],
        ),
        (true, false, _, true) => (
            [
                (point_x + 1) as f32,
                height_map[point_x + 1][point_z],
                point_z as f32,
            ],
            [
                point_x as f32,
                height_map[point_x][point_z - 1],
                (point_z - 1) as f32,
            ],
        ),
        (false, true, true, false) => (
            [
                (point_x - 1) as f32,
                height_map[point_x - 1][point_z],
                point_z as f32,
            ],
            [
                point_x as f32,
                height_map[point_x][point_z + 1],
                (point_z + 1) as f32,
            ],
        ),
        (_, _, _, _) => (
            [
                point_x as f32,
                height_map[point_x][point_z - 1],
                (point_z - 1) as f32,
            ],
            [
                (point_x - 1) as f32,
                height_map[point_x - 1][point_z],
                point_z as f32,
            ],
        ),
    };

    normalize(cross_vec3(v1, v2))
}

/// Calculates the cross vector product between two verctors (v1 X v2).
///
/// # Arguments
/// * `v1` - First vector.
/// * `v2` - Second vector.
///
fn cross_vec3(v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_normalize() {
        let arr = [2_f32, 0_f32, 0_f32];

        let normalized = normalize(arr);

        assert_ne!(normalized, arr);
        assert_eq!(normalized, [1_f32, 0_f32, 0_f32]);
    }

    #[test]
    fn should_cross_vec3() {
        let x = [1_f32, 0_f32, 0_f32];
        let y = [0_f32, 1_f32, 0_f32];

        assert_eq!([0_f32, 0_f32, 1_f32], cross_vec3(x, y));
    }
}
