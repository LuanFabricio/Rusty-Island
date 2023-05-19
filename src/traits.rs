pub trait Draw {
    fn new(title: &str) -> Self;
    fn draw_scene(&mut self);
}
