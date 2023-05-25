use super::*;

mod new {
    use super::*;

    #[test]
    fn should_create_a_entity() {
        const POSITION: [f32; 3] = [1_f32; 3];
        const ENTITY_TYPE: EntityType = EntityType::Animal1;

        let entity = Entity::new(POSITION, ENTITY_TYPE);

        assert_eq!(entity.rotation, 0_f32);
        assert_eq!(entity.entity_type, ENTITY_TYPE);
    }

    #[test]
    fn should_set_rotaion_to_zero_if_is_an_animal() {
        let animal1 = Entity::new([0_f32; 3], EntityType::Animal1);
        let animal2 = Entity::new([0_f32; 3], EntityType::Animal2);

        assert_eq!(animal1.rotation, 0_f32);
        assert_eq!(animal2.rotation, 0_f32);
    }

    #[test]
    fn should_set_rotaion_randomly_if_is_a_plant() {
        let plant1 = Entity::new([0_f32; 3], EntityType::Plant1);
        let plant2 = Entity::new([0_f32; 3], EntityType::Plant2);

        assert_ne!(plant1.rotation, plant2.rotation);
    }
}

mod get_type {
    use super::*;

    #[test]
    fn should_get_entity_type() {
        const ENTITY_TYPE: EntityType = EntityType::Plant1;
        let entity = Entity::new([1_f32; 3], ENTITY_TYPE);

        assert_eq!(entity.get_type(), ENTITY_TYPE);
    }
}

mod set_rotation {
    use super::*;

    #[test]
    fn should_update_rotation() {
        const NEW_ROTATION: f32 = 45_f32;

        let mut entity = Entity::new([1_f32; 3], EntityType::Animal2);

        entity.set_rotation(NEW_ROTATION);

        assert_eq!(entity.rotation, NEW_ROTATION);
    }
}

mod get_valid_position {
    use crate::utils::height_map::init_height_map;

    use super::*;

    #[test]
    fn should_be_a_scene_position() {
        let height_map = init_height_map::<5, 5>(1_f32);
        let tree_plants =
            tree_entity::TreeEntity::new(Entity::new([-100_f32; 3], EntityType::Animal1));
        let animals = vec![];

        let valid_possible_position = vec![
            (2, 2),
            (1, 2),
            (3, 2),
            (1, 1),
            (3, 1),
            (1, 3),
            (2, 3),
            (3, 3),
        ];

        assert_eq!(
            Entity::get_valid_position(
                &valid_possible_position,
                &height_map,
                &tree_plants,
                &animals
            )
            .len(),
            valid_possible_position.len(),
        );

        let invalid_possible_position = vec![
            (6, 6),
            (7, 6),
            (8, 6),
            (7, 7),
            (8, 7),
            (6, 8),
            (7, 8),
            (8, 8),
        ];

        assert_eq!(
            Entity::get_valid_position(
                &invalid_possible_position,
                &height_map,
                &tree_plants,
                &animals
            )
            .len(),
            0
        );
    }

    #[test]
    fn should_be_valid_land_position() {
        let invalid_height_map = init_height_map::<5, 5>(0_f32);
        let tree_plants =
            tree_entity::TreeEntity::new(Entity::new([-100_f32; 3], EntityType::Animal1));
        let animals = vec![];

        let possible_position = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];

        assert_eq!(
            Entity::get_valid_position(
                &possible_position,
                &invalid_height_map,
                &tree_plants,
                &animals
            )
            .len(),
            0
        );

        let valid_height_map = init_height_map::<5, 5>(1_f32);

        assert_eq!(
            Entity::get_valid_position(
                &possible_position,
                &valid_height_map,
                &tree_plants,
                &animals
            )
            .len(),
            8
        );
    }

    #[test]
    fn should_not_collide_with_a_plant() {
        let height_map = init_height_map::<5, 5>(1_f32);
        let tree_plants =
            tree_entity::TreeEntity::new(Entity::new([0_f32; 3], EntityType::Animal1));
        let animals = vec![];

        let possible_position = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];

        assert_eq!(
            Entity::get_valid_position(&possible_position, &height_map, &tree_plants, &animals)
                .len(),
            7
        );
    }

    #[test]
    fn should_not_collide_with_an_animal() {
        let height_map = init_height_map::<5, 5>(1_f32);
        let tree_plants =
            tree_entity::TreeEntity::new(Entity::new([-100_f32; 3], EntityType::Animal1));
        let animals = vec![Entity::new([0_f32; 3], EntityType::Animal2)];

        let possible_position = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];

        assert_eq!(
            Entity::get_valid_position(&possible_position, &height_map, &tree_plants, &animals)
                .len(),
            7
        );
    }
}

mod have_another_animal {
    use super::*;

    #[test]
    fn should_be_true_if_have_an_animal_in_same_position() {
        let position = [0_f32; 3];

        let animals = vec![Entity::new([0_f32; 3], EntityType::Animal1)];

        assert!(Entity::have_another_animal(position, &animals));
    }

    #[test]
    fn should_be_false_if_do_not_have_an_animal_in_same_position() {
        let position = [0_f32; 3];

        let animals = vec![Entity::new([1_f32; 3], EntityType::Animal1)];

        assert_eq!(Entity::have_another_animal(position, &animals), false);
    }
}

mod change_mode {
    use super::*;

    #[test]
    fn should_not_update_plant_mode() {
        let mut plant1 = Entity::new([0_f32; 3], EntityType::Plant1);
        let mut plant2 = Entity::new([1_f32; 3], EntityType::Plant2);

        let height_map = [[1_f32; 25]; 25];
        let tree_plants = TreeEntity::new(Entity::new([2_f32; 3], EntityType::Plant1));
        let animals = Vec::<Entity>::new();

        plant1.change_mode(&height_map, &tree_plants, &animals);
        assert_eq!(plant1.entity_mode, EntityMode::Idle);

        plant2.change_mode(&height_map, &tree_plants, &animals);
        assert_eq!(plant2.entity_mode, EntityMode::Idle);
    }

    #[test]
    fn should_update_idle_animals_mode() {
        let mut animal = Entity::new([0_f32; 3], EntityType::Animal1);

        let height_map = [[1_f32; 25]; 25];
        let tree_plants = TreeEntity::new(Entity::new([2_f32; 3], EntityType::Plant1));
        let animals = Vec::<Entity>::new();

        animal.change_mode(&height_map, &tree_plants, &animals);

        assert_ne!(animal.entity_mode, EntityMode::Idle);
    }

    #[test]
    fn should_move_animals_position_when_walking() {
        const INITIAL_POSITION: [f32; 3] = [0_f32; 3];
        let mut animal = Entity::new(INITIAL_POSITION, EntityType::Animal1);
        animal.entity_mode = EntityMode::Walking {
            target: (1_f32, 1_f32),
        };

        let height_map = [[1_f32; 25]; 25];
        let tree_plants = TreeEntity::new(Entity::new([2_f32; 3], EntityType::Plant1));
        let animals = Vec::<Entity>::new();

        animal.change_mode(&height_map, &tree_plants, &animals);

        assert_ne!(animal.entity_mode, EntityMode::Idle);
        assert_ne!(animal.position, INITIAL_POSITION);
    }

    #[test]
    fn should_be_idle_if_the_target_is_actived() {
        const INITIAL_POSITION: [f32; 3] = [0_f32; 3];
        let mut animal = Entity::new(INITIAL_POSITION, EntityType::Animal1);
        animal.entity_mode = EntityMode::Walking {
            target: (INITIAL_POSITION[0], INITIAL_POSITION[2]),
        };

        let height_map = [[1_f32; 25]; 25];
        let tree_plants = TreeEntity::new(Entity::new([2_f32; 3], EntityType::Plant1));
        let animals = Vec::<Entity>::new();

        animal.change_mode(&height_map, &tree_plants, &animals);

        assert_eq!(animal.entity_mode, EntityMode::Idle);
        assert_eq!(animal.position, INITIAL_POSITION);
    }
}
