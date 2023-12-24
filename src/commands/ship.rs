use crate::{
    components::{Enemy, Health, Player, Ship},
    systems::ship::{SHIP_INITIAL_HEALTH, SHIP_SIZE},
};
use bevy::{ecs::system::Command, prelude::*, sprite::MaterialMesh2dBundle};

pub enum ShipKind {
    Player,
    Enemy,
}

pub struct SpawnShipCommand {
    pub kind: ShipKind,
    pub position: Vec3,
}

impl Command for SpawnShipCommand {
    fn apply(self, world: &mut World) {
        let mesh_handle = world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
            let shape = shape::Cube { size: SHIP_SIZE };
            meshes.add(Mesh::from(shape))
        });

        let material_handle =
            world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
                let material = ColorMaterial::from(Color::WHITE);
                materials.add(material)
            });

        let bundle = MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_translation(self.position),
            ..default()
        };

        match self.kind {
            ShipKind::Player => {
                world.spawn((bundle, Ship, Health(SHIP_INITIAL_HEALTH), Player));
            }
            ShipKind::Enemy => {
                world.spawn((bundle, Ship, Health(SHIP_INITIAL_HEALTH), Enemy));
            }
        };
    }
}
