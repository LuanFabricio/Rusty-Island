use rand::{rngs::ThreadRng, Rng};

#[cfg(test)]
mod test;

const LAND_VALUE: f32 = 1_f32;

/// Create a height map with a default value.
/// # Arguments
/// * `default_value` - The default value that will be used in the height map.
///
/// # Examples
/// ```
/// use utils::height_map::*;
///
/// let height_map = init_height_map::<2, 2>(0_f32);
/// assert_eq!(height_map, [[0_f32; 2]; 2]);
/// ```
pub fn init_height_map<const W: usize, const H: usize>(default_value: f32) -> [[f32; H]; W] {
    [[default_value; H]; W]
}

/// Create land based on size (starts in the middle of the map).
/// # Arguments
/// * `height_map` - Matrix with initial height_map.
/// * `max_size` - Total size of the island.
pub fn create_land<const W: usize, const H: usize>(
    height_map: &mut [[f32; H]; W],
    max_size: usize,
) {
    let mut rand = rand::thread_rng();

    let half_w = W / 2;
    let half_h = H / 2;

    height_map[half_w][half_h] = LAND_VALUE;

    let mut land_count = 1;
    while land_count < max_size {
        let (x, y) = get_height_map_position(height_map, &mut rand);

        height_map[x][y] = LAND_VALUE;
        land_count += 1;
    }
}

/// Get a valid position for create a land based on height map.
/// # Arguments
/// * `height_map` - Matrix with height map data.
/// * `rand` - ThreadRng for generate randomly coords.
///
/// # Examples
/// ```
/// use rand::Rng;
/// use utils::height_map::*;
///
/// let mut rand = rand::thread_rng();
/// let mut height_map = init_height_map::<3, 3>(0);
/// height_map[0][0] = 1;
///
/// let (x, y) = get_height_map_position();
///
/// assert!(
///     (x == 0 && y == 1)
///     || (x == 1 && y == 0)
/// );
/// ```
fn get_height_map_position<const W: usize, const H: usize>(
    height_map: &[[f32; H]; W],
    rand: &mut ThreadRng,
) -> (usize, usize) {
    let mut x = rand.gen_range(0..W);
    let mut y = rand.gen_range(0..H);

    while !is_valid_height_map_position(x as isize, y as isize, height_map) {
        x = rand.gen_range(0..W);
        y = rand.gen_range(0..H);
    }

    (x, y)
}

/// Indicate if a position is valid on a height map or not.
/// # Arguments
/// * `x` - An isize of a position in height map.
/// * `y` - An isize of a position in height map.
/// * `height_map` - The height_map that will be used to know if is a valid position.
///
/// # Examples
/// ```
/// use utils::height_map::*;
/// let mut height_map = init_height_map::<3, 3>(0);
/// height_map[1][1] = 1_f32;
///
/// let is_valid = is_valid_height_map_position(1, 1);
/// assert!(!is_valid);
/// let is_valid = is_valid_height_map_position(1, 0);
/// assert!(is_valid);
/// ```
fn is_valid_height_map_position<const W: usize, const H: usize>(
    x: isize,
    y: isize,
    height_map: &[[f32; H]; W],
) -> bool {
    let neightbors = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];

    let mut have_land_near = false;
    for (neightbor_x, neightbor_y) in neightbors {
        let valid_coords = neightbor_x >= 0
            && neightbor_x < W as isize
            && neightbor_y >= 0
            && neightbor_y < H as isize;
        if valid_coords && height_map[neightbor_x as usize][neightbor_y as usize] == LAND_VALUE {
            have_land_near = true;
            break;
        }
    }

    have_land_near && height_map[x as usize][y as usize] != LAND_VALUE
}
