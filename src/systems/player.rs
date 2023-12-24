use super::ship::SHIP_SPEED;
use crate::{
    commands::{
        laser::SpawnLaserCommand,
        ship::{ShipKind, SpawnShipCommand},
    },
    components::Player,
};
use bevy::{prelude::*, window::PrimaryWindow};
use std::ops::Add;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (rotate_ship_to_cursor, handle_keyboard_input));
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

    let world_position = camera
        .viewport_to_world_2d(camera_transform, target)
        .unwrap();
    let mut ship_transform = query.single_mut();
    let pos = ship_transform.translation.truncate();

    let direction = world_position - pos;

    // Calculate angle between direction and x-axis
    let angle = direction.y.atan2(direction.x);

    ship_transform.rotation = Quat::from_rotation_z(angle);
}

fn handle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, Entity), With<Player>>,
    mut commands: Commands,
) {
    let Ok((mut ship_transform, ship_entity)) = query.get_single_mut() else {
        return;
    };

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
        let direction = ship_transform.rotation.xyz().normalize();
        let laser_position = ship_transform
            .translation
            .clone()
            .add(Vec3::new(0., 50., 0.));

        commands.add(SpawnLaserCommand {
            direction,
            position: laser_position,
            spawned_by: ship_entity,
        });
    }
}
