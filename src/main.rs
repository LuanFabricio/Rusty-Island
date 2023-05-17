mod utils;

fn main() {
    let mut hm = utils::height_map::init_height_map::<10, 20>(-1_f32);
    println!("{hm:?}");

    let shm = utils::height_map::smooth_height_map(hm);
    println!("{shm:?}");

    utils::height_map::create_land(&mut hm, 150);
    println!("{hm:?}");

    utils::height_map::create_lakes(&mut hm, 20);
    utils::height_map::print_height_map(&utils::height_map::smooth_height_map(hm));
    println!("Hello, world!");
}
