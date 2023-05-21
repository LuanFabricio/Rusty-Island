use glium::implement_vertex;

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);
