use crate::{
    components::{Laser, LaserCooldown},
    systems::laser::{LASER_COOLDOWN_DURATION, LASER_SIZE},
};
use bevy::{
    ecs::system::Command, math::primitives::Rectangle, prelude::*, sprite::MaterialMesh2dBundle,
};

pub struct SpawnLaserCommand {
    pub transform: Transform,
    pub spawned_by_entity: Entity,
}

impl Command for SpawnLaserCommand {
    fn apply(self, world: &mut World) {
        let entity_has_cooldown = world.get::<LaserCooldown>(self.spawned_by_entity).is_some();

        if entity_has_cooldown {
            return;
        }

        let mesh_handle = world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
            let shape = Rectangle {
                half_size: Vec2::new(LASER_SIZE / 2., LASER_SIZE / 2.),
            };
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
                transform: self.transform,
                ..default()
            },
            Laser,
        ));
        world
            .entity_mut(self.spawned_by_entity)
            .insert(LaserCooldown(Timer::from_seconds(
                LASER_COOLDOWN_DURATION,
                TimerMode::Once,
            )));
    }
}
