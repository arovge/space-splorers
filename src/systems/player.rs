use super::{cursor_position_to_world_position, ship::SHIP_SPEED};
use crate::{
    commands::{
        laser::SpawnLaserCommand,
        ship::{ShipKind, SpawnShipCommand},
    },
    components::Player,
};
use bevy::{prelude::*, window::PrimaryWindow};

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
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Some(target_position) = cursor_position_to_world_position(&window_query, &camera_query)
    else {
        return;
    };
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };
    let ship_position = player_transform.translation.xy();

    let direction = target_position - ship_position;

    // Calculate angle between direction and x-axis
    let angle = direction.y.atan2(direction.x);
    player_transform.rotation = Quat::from_rotation_z(angle);
}

fn handle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, Entity), With<Player>>,
    mut commands: Commands,
) {
    let Ok((mut player_transform, entity)) = player_query.get_single_mut() else {
        return;
    };

    if keys.pressed(KeyCode::W) {
        player_transform.translation.y += SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        player_transform.translation.x -= SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::S) {
        player_transform.translation.y -= SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        player_transform.translation.x += SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Space) {
        let offset_distance = 100.0;
        let object_b_position = player_transform.translation
            + player_transform.rotation * Vec3::new(offset_distance, 0.0, 0.0);

        commands.add(SpawnLaserCommand {
            transform: Transform::from_translation(object_b_position),
            spawned_by_entity: entity,
        });
    }
}
