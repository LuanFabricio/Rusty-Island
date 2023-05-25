#[cfg(test)]
mod test;

use crate::scene::mesh_map::MeshMap;

use super::{Entity, EntityType};

#[derive(Debug, PartialEq)]
pub struct TreeEntity {
    pub key: Entity,
    leafs: [Option<Box<TreeEntity>>; 4],
}

impl TreeEntity {
    pub fn new(key: Entity) -> Self {
        Self {
            key,
            leafs: [None, None, None, None],
        }
    }

    pub fn add(&mut self, child: Entity) {
        let leaf = match (
            self.key.position[0] < child.position[0],
            self.key.position[2] < child.position[2],
        ) {
            (true, true) => &mut self.leafs[0],
            (true, false) => &mut self.leafs[1],
            (false, true) => &mut self.leafs[2],
            (false, false) => &mut self.leafs[3],
        };

        if let Some(c) = leaf {
            c.add(child);
        } else {
            *leaf = Some(Box::new(TreeEntity::new(child)));
        }
    }

    pub fn draw(
        &self,
        mesh_map: &mut MeshMap,
        frame: &mut glium::Frame,
        uniforms: ([[f32; 4]; 4], [[f32; 4]; 4], [f32; 3]),
        params: &glium::DrawParameters,
    ) {
        let mesh = match self.key.get_type() {
            EntityType::Plant1 => &mut mesh_map.plant1,
            EntityType::Plant2 => &mut mesh_map.plant2,
            _ => return,
        };

        mesh.set_position(self.key.position);
        mesh.set_rotation_y(self.key.rotation);
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
        );

        for leaf in self.leafs.iter() {
            if let Some(child) = leaf {
                child.draw(mesh_map, frame, uniforms, params);
            }
        }
    }

    pub fn applly_function(&self, function: &dyn Fn(&TreeEntity)) {
        function(self);

        for leaf in self.leafs.iter() {
            if let Some(child) = leaf {
                child.applly_function(function);
            }
        }
    }

    pub fn collide(&self, position: [f32; 3]) -> bool {
        // TODO: Change to collision box.
        if self.key.position[0] == position[0] && self.key.position[2] == position[2] {
            return true;
        }

        for leaf in self.leafs.iter() {
            if let Some(child) = leaf {
                if child.collide(position) {
                    return true;
                }
            }
        }

        false
    }
}
