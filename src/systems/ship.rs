use super::laser::{LASER_COOLDOWN_DURATION, LASER_SIZE};
use crate::{
    commands::{
        explosion::SpawnExplosionCommand,
        laser::SpawnLaserCommand,
        ship::{ShipKind, SpawnShipCommand},
    },
    components::{Health, Laser, LaserCooldown, Player, Ship},
};
use bevy::{prelude::*, window::PrimaryWindow};
use std::ops::Add;

const SHIP_SPEED: f32 = 150.;
pub const SHIP_SIZE: f32 = 20.;
pub const SHIP_INITIAL_HEALTH: usize = 100;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                handle_keyboard_input,
                check_for_laser_hit,
                check_health_to_despawn,
                check_for_enemy_spawn,
                rotate_ship_to_cursor,
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.add(SpawnShipCommand {
        kind: ShipKind::Player,
        position: Vec3::ZERO,
    });
}

fn rotate_ship_to_cursor(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let cursor_position = windows.get_single().ok().and_then(|a| a.cursor_position());
    let Some(target) = cursor_position else {
        return;
    };

    let ray = camera.viewport_to_world_2d(camera_transform, target);
    if let Some(ray) = ray {
        let mut ship_transform = query.single_mut();
        let pos = ship_transform.translation.truncate();

        let direction = ray - pos;

        // Calculate angle between direction and x-axis
        let angle = direction.y.atan2(direction.x);

        ship_transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn handle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, Entity, &mut Ship), With<Player>>,
    mut commands: Commands,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    cooldowns: Query<&LaserCooldown, With<Player>>,
) {
    let (mut ship_transform, ship_entity, _ship) = query.single_mut();

    if keys.pressed(KeyCode::W) {
        ship_transform.translation.y += SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        ship_transform.translation.x -= SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::S) {
        ship_transform.translation.y -= SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        ship_transform.translation.x += SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Space) {
        let has_cooldown = cooldowns
            .get_component::<LaserCooldown>(ship_entity)
            .is_ok();

        if !has_cooldown {
            let direction = ship_transform.rotation.xyz().normalize();
            let laser_position = ship_transform
                .translation
                .clone()
                .add(Vec3::new(0., 50., 0.));

            commands.add(SpawnLaserCommand {
                direction,
                position: laser_position,
            });
            commands
                .entity(ship_entity)
                .insert(LaserCooldown(Timer::from_seconds(
                    LASER_COOLDOWN_DURATION,
                    TimerMode::Once,
                )));
        }
    }

    if keys.pressed(KeyCode::Q) {
        app_exit_events.send(bevy::app::AppExit);
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
