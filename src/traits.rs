pub trait Draw {
    fn new(title: &str) -> Self;
    fn draw_scene<const W: usize, const H: usize>(&mut self, height_map: [[f32; H]; W]);
}
