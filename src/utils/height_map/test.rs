use super::*;

mod init_height_map {
    use super::*;

    #[test]
    fn should_start_with_default_value() {
        const DEFAULT_VALUE: f32 = 1_f32;

        let height_map = init_height_map::<3, 4>(DEFAULT_VALUE);

        assert_eq!(height_map, [[DEFAULT_VALUE; 4]; 3]);
    }
}

mod create_land {
    use super::*;

    #[test]
    fn should_create_correct_amount_of_land() {
        const LAND_SIZE: usize = 5;

        let mut height_map = init_height_map::<4, 4>(0_f32);

        create_land(&mut height_map, LAND_SIZE);

        let mut count_land = 0;

        for row in height_map {
            for item in row {
                if item == LAND_VALUE {
                    count_land += 1;
                }
            }
        }

        assert_eq!(count_land, LAND_SIZE);
    }
}

mod get_height_map_position {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn should_return_valid_position() {
        const W: usize = 3;
        const H: usize = 3;

        let mut rand = thread_rng();
        let mut height_map = init_height_map::<W, H>(0_f32);

        height_map[0][0] = LAND_VALUE;

        let (pos_x, pos_y) = get_height_map_position(&height_map, &mut rand);

        // X and Y index in height_map matrix.
        assert!(pos_x < W);
        assert!(pos_y < H);

        // Should not be a land value.
        assert!(height_map[pos_x][pos_y] != LAND_VALUE);

        // Should be neightbor of a land
        assert!((pos_x == 0 && pos_y == 1) || (pos_x == 1 && pos_y == 0))
    }
}

mod smooth_height_map {
    use super::*;

    #[test]
    fn should_change_the_map() {
        let mut height_map = init_height_map::<3, 3>(0_f32);
        height_map[1][1] = 1_f32;

        let smooth_height_map = smooth_height_map(height_map);

        assert_ne!(smooth_height_map, height_map);
    }
}

mod create_lakes {
    use super::*;

    #[test]
    fn should_create_lakes() {
        let mut height_map = init_height_map::<5, 5>(LAND_VALUE);

        const EXPECTED_TOTAL_LAKE: usize = 5;
        create_lakes(&mut height_map, EXPECTED_TOTAL_LAKE);

        let mut total_lake = 0_usize;

        for row in height_map {
            for item in row {
                if item == LAKE_VALUE {
                    println!("total_lake");
                    total_lake += 1;
                }
            }
        }

        assert_eq!(total_lake, EXPECTED_TOTAL_LAKE);
    }
}
