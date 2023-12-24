use super::laser::LASER_SIZE;
use crate::{
    commands::{
        explosion::SpawnExplosionCommand,
        ship::{ShipKind, SpawnShipCommand},
    },
    components::{Health, Laser, Ship},
};
use bevy::prelude::*;

pub const SHIP_SPEED: f32 = 150.;
pub const SHIP_SIZE: f32 = 20.;
pub const SHIP_INITIAL_HEALTH: usize = 100;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_for_laser_hit,
                check_health_to_despawn,
                check_for_enemy_spawn,
            ),
        );
    }
}

fn check_for_laser_hit(
    mut ship_query: Query<(&mut Health, &Transform), With<Health>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    mut commands: Commands,
) {
    for (laser_entity, laser_transform) in &laser_query {
        for (mut ship_health, ship_transform) in ship_query.iter_mut() {
            let is_colliding = {
                // assumes ship and laser are both spheres (they're cubes)
                // need a more complex collision system for diff shapes
                let dist = laser_transform
                    .translation
                    .xy()
                    .distance(ship_transform.translation.xy());
                dist < (SHIP_SIZE / 2.) + (LASER_SIZE / 2.)
            };
            if is_colliding {
                commands.entity(laser_entity).despawn();
                ship_health.take_damange();
            }
        }
    }
}

fn check_health_to_despawn(
    ship_query: Query<(&Transform, Entity, &Health), With<Ship>>,
    mut commands: Commands,
) {
    for (ship_transform, ship_entity, health) in &ship_query {
        if health.0 <= 0 {
            commands.entity(ship_entity).despawn();
            commands.add(SpawnExplosionCommand {
                position: ship_transform.translation,
            });
        }
    }
}

fn check_for_enemy_spawn(keys: Res<Input<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::F) {
        commands.add(SpawnShipCommand {
            kind: ShipKind::Enemy,
            position: Vec3::ZERO,
        });
    }
}
