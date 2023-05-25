mod entity;
mod mesh_map;
#[cfg(test)]
mod test;

use crate::render::glium::mesh::Mesh;
use crate::render::obj_reader;
use crate::utils::height_map::{create_land, init_height_map, smooth_height_map};
use rand::Rng;

pub use entity::{Entity, EntityType};

use self::mesh_map::MeshMap;

pub struct Scene<const W: usize, const H: usize> {
    width: usize,
    height: usize,
    height_map: [[f32; H]; W],
    mesh_map: MeshMap,
    plants: Option<entity::tree_entity::TreeEntity>,
    animals: Vec<entity::Entity>,
}

impl<const W: usize, const H: usize> Scene<W, H> {
    pub fn new(display: &glium::Display) -> Self {
        let mut height_map = init_height_map::<W, H>(-2_f32);
        create_land(&mut height_map, (W as f32 * H as f32 * 0.75_f32) as usize);

        Self {
            width: W,
            height: H,
            height_map: smooth_height_map(height_map),
            plants: None,
            animals: vec![],
            mesh_map: Self::load_mesh_map(display),
        }
    }

    fn load_mesh_map(display: &glium::Display) -> MeshMap {
        let plant1_obj = obj_reader::ObjReader::new("assets/plant1.obj").unwrap();
        let plant2_obj = obj_reader::ObjReader::new("assets/plant2.obj").unwrap();
        let animal1_obj = obj_reader::ObjReader::new("assets/animal1.obj").unwrap();

        MeshMap {
            plant1: Mesh::from_obj(plant1_obj.get_obj(), display),
            plant2: Mesh::from_obj(plant2_obj.get_obj(), display),
            animal1: Mesh::from_obj(animal1_obj.get_obj(), display),
            animal2: Mesh::from_obj(animal1_obj.get_obj(), display),
        }
    }

    pub fn create_entities(&mut self, quantity: usize, entity_type: EntityType) {
        for _ in 0..quantity {
            let (x, z) = self.get_avaliable_position();

            self.add_entity(Entity::new([x as f32, 0_f32, z as f32], entity_type));
        }
    }

    fn get_avaliable_position(&self) -> (usize, usize) {
        let mut rand = rand::thread_rng();

        let mut x = rand.gen_range(0..self.width);
        let mut z = rand.gen_range(0..self.height);

        while self.height_map[x][z] < crate::utils::height_map::LAND_VALUE && !self.collides((x, z))
        {
            x = rand.gen_range(0..self.width);
            z = rand.gen_range(0..self.height);
        }

        (x, z)
    }

    fn collides(&self, (x, z): (usize, usize)) -> bool {
        if let Some(plants) = &self.plants {
            if plants.collide([x as f32, 0_f32, z as f32]) {
                return true;
            }
        }

        for animal in self.animals.iter() {
            if animal.position[0] == x as f32 && animal.position[2] == z as f32 {
                return true;
            }
        }

        false
    }

    pub fn add_entity(&mut self, mut entity: Entity) {
        self.fix_position(&mut entity);
        match entity.get_type() {
            EntityType::Plant1 | EntityType::Plant2 => {
                self.add_plant(entity);
            }
            EntityType::Animal1 | EntityType::Animal2 => self.animals.push(entity),
        }
    }

    fn fix_position(&self, entity: &mut Entity) {
        let [x, _, z] = entity.position;

        let valid_x = x >= 0_f32 && x < self.width as f32;
        let valid_z = z >= 0_f32 && z < self.height as f32;
        if valid_x && valid_z {
            entity.position[1] = self.height_map[x as usize][z as usize];
        }
    }

    fn add_plant(&mut self, entity: Entity) {
        if let Some(first_node) = &mut self.plants {
            first_node.add(entity);
        } else {
            self.plants = Some(entity::tree_entity::TreeEntity::new(entity));
        }
    }

    pub fn get_height_map_mesh(&self, display: &glium::Display) -> Mesh {
        let mesh_colors = (
            [0.25_f32, 0.25_f32, 0.25_f32],
            [1_f32, 0_f32, 0_f32],
            [0_f32, 0_f32, 0_f32],
        );
        crate::render::glium::util::height_map_to_mesh(self.height_map, mesh_colors, display)
    }

    pub fn move_animals(&mut self) {
        let mut plants =
            &entity::tree_entity::TreeEntity::new(Entity::new([-1000_f32; 3], EntityType::Plant1));

        if let Some(p) = &self.plants {
            plants = p
        };

        let animals = self.animals.clone();

        for animal in self.animals.iter_mut() {
            animal.change_mode(&self.height_map, plants, &animals);
        }
    }

    pub fn draw_entities(
        &mut self,
        frame: &mut glium::Frame,
        uniforms: ([[f32; 4]; 4], [[f32; 4]; 4], [f32; 3]),
        params: &glium::DrawParameters,
    ) {
        if let Some(plants) = &self.plants {
            plants.draw(&mut self.mesh_map, frame, uniforms, params);
        }

        for animal in self.animals.iter() {
            let mesh = match animal.get_type() {
                EntityType::Animal1 => &mut self.mesh_map.animal1,
                EntityType::Animal2 => &mut self.mesh_map.animal1,
                _ => break,
            };

            mesh.set_position(animal.position);
            // BUG: Dont rotates properly.
            mesh.set_rotation_y(animal.rotation);

            mesh.draw(
                frame,
                &glium::uniform! {
                    view: uniforms.0,
                    perspective: uniforms.1,
                    u_light: uniforms.2,
                    matrix: mesh.matrix,
                    ambient_color: mesh.ambient,
                    diffuse_color: mesh.diffuse,
                    specular_color: mesh.specular,
                },
                params,
            )
        }
    }
}
