use super::*;

mod new {
    use super::*;

    #[test]
    fn should_create_a_entity() {
        const POSITION: [f32; 3] = [1_f32; 3];
        const ENTITY_TYPE: EntityType = EntityType::Animal1;

        let entity = Entity::new(POSITION, ENTITY_TYPE);

        assert_eq!(entity.rotation, 0_f32);
        assert_eq!(entity.position, POSITION);
        assert_eq!(entity.entity_type, ENTITY_TYPE);
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
