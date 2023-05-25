use super::*;

mod new {
    use super::*;

    #[test]
    fn should_create() {
        let entity = Entity::new([0_f32; 3], EntityType::Plant1);
        let tree = TreeEntity::new(entity);

        assert_eq!(tree.key, entity);
    }

    #[test]
    fn should_initialize_leafs_with_None() {
        let entity = Entity::new([0_f32; 3], EntityType::Plant1);
        let tree = TreeEntity::new(entity);

        assert_eq!(tree.leafs, [None, None, None, None]);
    }
}
