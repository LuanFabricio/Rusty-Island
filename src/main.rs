mod utils;

fn main() {
    let mut hm = utils::height_map::init_height_map::<3, 3>(0_f32);
    println!("{hm:?}");
    utils::height_map::create_land(&mut hm, 4);
    println!("{hm:?}");
    println!("Hello, world!");
}
