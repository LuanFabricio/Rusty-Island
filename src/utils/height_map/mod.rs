use rand::{rngs::ThreadRng, Rng};

#[cfg(test)]
mod test;

const LAND_VALUE: f32 = 1_f32;
const LAKE_VALUE: f32 = 0_f32;
const SEA_VALUE: f32 = -1_f32;

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

/// Create a new height map based on a mask (using box blur).
///
/// #Arguments
/// * `height_map` - Height map that will be used to create a smooth version.
pub fn smooth_height_map<const W: usize, const H: usize>(
    height_map: [[f32; H]; W],
) -> [[f32; H]; W] {
    const NEIGHTBOR_WEIGHT: f32 = 1_f32 / 9_f32;
    const POINT_WEIGHT: f32 = 1_f32 / 3_f32;

    let mask: [[f32; 3]; 3] = [
        [NEIGHTBOR_WEIGHT, NEIGHTBOR_WEIGHT, NEIGHTBOR_WEIGHT],
        [NEIGHTBOR_WEIGHT, POINT_WEIGHT, NEIGHTBOR_WEIGHT],
        [NEIGHTBOR_WEIGHT, NEIGHTBOR_WEIGHT, NEIGHTBOR_WEIGHT],
    ];

    let mut new_height_map = height_map;

    for i in 0..W {
        for j in 0..H {
            new_height_map[i][j] = apply_mask(&height_map, &mask, i, j);
        }
    }

    new_height_map
}

fn apply_mask<const W: usize, const H: usize>(
    matrix: &[[f32; H]; W],
    mask: &[[f32; 3]; 3],
    x: usize,
    y: usize,
) -> f32 {
    let neightbors = [
        (x as isize - 1, y as isize - 1, mask[0][0]),
        (x as isize, y as isize - 1, mask[1][0]),
        (x as isize + 1, y as isize - 1, mask[2][0]),
        (x as isize - 1, y as isize, mask[0][1]),
        (x as isize, y as isize, mask[1][1]),
        (x as isize + 1, y as isize, mask[2][1]),
        (x as isize - 1, y as isize + 1, mask[0][2]),
        (x as isize, y as isize + 1, mask[1][2]),
        (x as isize + 1, y as isize + 1, mask[2][2]),
    ];

    let mut result = 0_f32;

    for (n_x, n_y, weight) in neightbors {
        let valid_x = n_x >= 0 && n_x < W as isize;
        let valid_y = n_y >= 0 && n_y < H as isize;

        if valid_x && valid_y {
            result += matrix[n_x as usize][n_y as usize] * weight;
        }
    }

    result
}

pub fn create_lakes<const W: usize, const H: usize>(
    height_map: &mut [[f32; H]; W],
    mut total_lake: usize,
) {
    const MIN_LAKE_SIZE: usize = 3;
    total_lake = total_lake.max(MIN_LAKE_SIZE + 1);

    let mut rand = rand::thread_rng();

    let mut height_map_total_lake = 0_usize;
    while height_map_total_lake < total_lake {
        let lake_size = rand
            .gen_range(MIN_LAKE_SIZE..total_lake)
            .min(total_lake - height_map_total_lake);

        // println!(
        //     "create_lakes {:?} - {:?}",
        //     height_map_total_lake, total_lake
        // );

        let (init_x, init_y) = get_valid_lake_position(height_map, &mut rand);

        create_lake(height_map, lake_size, init_x, init_y, &mut rand);

        height_map_total_lake += lake_size;
    }
}

fn get_valid_lake_position<const W: usize, const H: usize>(
    height_map: &mut [[f32; H]; W],
    rand: &mut ThreadRng,
) -> (usize, usize) {
    let mut x = rand.gen_range(0..W);
    let mut y = rand.gen_range(0..H);

    while !is_valid_lake_position(height_map, x, y) {
        println!("{x} {y} - {}", is_valid_lake_position(height_map, x, y));
        x = rand.gen_range(0..W);
        y = rand.gen_range(0..H);
    }

    (x, y)
}

fn is_valid_lake_position<const W: usize, const H: usize>(
    height_map: &[[f32; H]; W],
    x: usize,
    y: usize,
) -> bool {
    let valid_coords = x < W && y < H;

    if !valid_coords {
        return false;
    }

    let neightbors = [
        (x as isize - 1, y as isize - 1),
        (x as isize, y as isize - 1),
        (x as isize + 1, y as isize - 1),
        (x as isize - 1, y as isize),
        (x as isize + 1, y as isize),
        (x as isize - 1, y as isize + 1),
        (x as isize, y as isize + 1),
        (x as isize + 1, y as isize + 1),
    ];

    for (n_x, n_y) in neightbors {
        let valid_neightbor = n_x >= 0 && n_x < W as isize && n_y >= 0 && n_y < H as isize;

        if valid_neightbor && height_map[n_x as usize][n_y as usize] == SEA_VALUE {
            return false;
        }
    }

    true
}

fn create_lake<const W: usize, const H: usize>(
    height_map: &mut [[f32; H]; W],
    mut lake_size: usize,
    init_x: usize,
    init_y: usize,
    rand: &mut ThreadRng,
) {
    height_map[init_x][init_y] = LAKE_VALUE;
    lake_size -= 1;

    let mut possible_points: Vec<(usize, usize)> =
        get_valid_neightbors::<W, H>(init_x as isize, init_y as isize);

    while lake_size > 0 && possible_points.len() > 0 {
        // println!("lake_size: {lake_size}");
        // println!("{:?}", 0..possible_points.len());
        // println!("{:?}", possible_points);
        let point_index = rand.gen_range(0..possible_points.len());

        let (x, y) = possible_points[point_index];

        // println!("{x} {y} {}", is_valid_lake_position(height_map, x, y));
        if is_valid_lake_position(height_map, x, y) {
            height_map[x][y] = LAKE_VALUE;

            let valid_neightbors = get_valid_neightbors::<W, H>(x as isize, y as isize);

            for neighbor in valid_neightbors {
                possible_points.push(neighbor);
            }

            lake_size -= 1;
        }

        possible_points.remove(point_index);
    }
    // println!("lake_size: {lake_size}");

    // println!("{height_map:?}");
}

fn get_valid_neightbors<const W: usize, const H: usize>(x: isize, y: isize) -> Vec<(usize, usize)> {
    let mut valid_neightbors: Vec<(usize, usize)> = vec![];

    let neightbors = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    for (n_x, n_y) in neightbors {
        let valid_coords = n_x >= 0 && x < W as isize && n_y >= 0 && y < H as isize;

        if valid_coords {
            valid_neightbors.push((n_x as usize, n_y as usize));
        }
    }

    valid_neightbors
}

pub fn print_height_map<const W: usize, const H: usize>(height_map: &[[f32; H]; W]) {
    for row in height_map.iter() {
        for item in row {
            print!("{item} ");
        }
        println!();
    }
}
