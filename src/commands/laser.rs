use crate::{components::Laser, systems::laser::LASER_SIZE};
use bevy::{ecs::system::Command, prelude::*, sprite::MaterialMesh2dBundle};

pub struct SpawnLaserCommand {
    pub direction: Vec3,
    pub position: Vec3,
}

impl Command for SpawnLaserCommand {
    fn apply(self, world: &mut World) {
        let mesh_handle = world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
            let shape = shape::Cube { size: LASER_SIZE };
            meshes.add(Mesh::from(shape))
        });

        let material_handle =
            world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
                let material = ColorMaterial::from(Color::WHITE);
                materials.add(material)
            });

        world.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                transform: Transform::from_translation(self.position),
                ..default()
            },
            Laser {
                direction: Vec3::ZERO,
            },
        ));
    }
}
