use rand::Rng;

use self::tree_entity::TreeEntity;

#[cfg(test)]
mod test;
pub mod tree_entity;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EntityMode {
    Idle,
    Walking { target: (f32, f32) },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EntityType {
    Animal1,
    Animal2,
    Plant1,
    Plant2,
}

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub position: [f32; 3],
    pub rotation: f32,
    entity_type: EntityType,
    entity_mode: EntityMode,
}

impl Entity {
    pub fn new(position: [f32; 3], entity_type: EntityType) -> Self {
        Self {
            position,
            rotation: 0_f32,
            entity_type,
            entity_mode: EntityMode::Idle,
        }
    }

    pub fn get_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }

    pub fn walk<const W: usize, const H: usize>(
        &mut self,
        height_map: &[[f32; H]; W],
        tree_plants: &TreeEntity,
        animals: &Vec<Entity>,
    ) {
        if self.entity_type == EntityType::Plant1
            || self.entity_type == EntityType::Plant2
            || self.entity_mode != EntityMode::Idle
        {
            return;
        }

        let current_x = self.position[0] as isize;
        let current_z = self.position[2] as isize;
        let possible_position = vec![
            (current_x + 1, current_z),
            (current_x + 1, current_z + 1),
            (current_x, current_z + 1),
            (current_x - 1, current_z + 1),
            (current_x - 1, current_z),
            (current_x - 1, current_z - 1),
            (current_x, current_z - 1),
            (current_x + 1, current_z - 1),
        ];
        let valid_positions =
            Self::get_valid_position(&possible_position, height_map, tree_plants, animals);

        if valid_positions.len() == 0 {
            return;
        }

        let mut rand = rand::thread_rng();
        let index = rand.gen_range(0..valid_positions.len());

        let position_index = valid_positions[index];
        let x = possible_position[position_index].0 as usize;
        let z = possible_position[position_index].1 as usize;

        self.position = [x as f32, height_map[x][z], z as f32];
        self.set_rotation(position_index as f32 * 45_f32);
    }

    fn get_valid_position<const W: usize, const H: usize>(
        possible_position: &Vec<(isize, isize)>,
        height_map: &[[f32; H]; W],
        tree_plants: &TreeEntity,
        animals: &Vec<Entity>,
    ) -> Vec<usize> {
        const VALID_HEIGHT: f32 = 1_f32;

        let mut valid_positions = Vec::<usize>::new();
        for i in 0..possible_position.len() {
            let (x, z) = possible_position[i];
            let valid_x = x >= 0 && x < W as isize;
            let valid_z = z >= 0 && z < H as isize;

            let pos = [x as f32, 0_f32, z as f32];
            if valid_x
                && valid_z
                && height_map[x as usize][z as usize] >= VALID_HEIGHT
                && !tree_plants.collide(pos)
                && !Self::have_another_animal(pos, animals)
            {
                valid_positions.push(i);
            }
        }

        valid_positions
    }

    fn have_another_animal(position: [f32; 3], animals: &Vec<Entity>) -> bool {
        for animal in animals.iter() {
            let [animal_x, _, animal_z] = animal.position;

            if animal_x == position[0] && animal_z == position[2] {
                return true;
            }
        }

        false
    }
}
