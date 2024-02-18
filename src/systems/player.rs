use super::{get_world_pos, ship::SHIP_SPEED};
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
    let Some(target_position) = get_world_pos(&window_query, &camera_query) else {
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
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, Entity), With<Player>>,
    mut commands: Commands,
) {
    let Ok((mut player_transform, entity)) = player_query.get_single_mut() else {
        return;
    };

    let is_boosted = keys.pressed(KeyCode::ShiftLeft);
    let speed = if is_boosted {
        SHIP_SPEED * 2.
    } else {
        SHIP_SPEED
    };

    if keys.pressed(KeyCode::KeyW) {
        // let delta = time.delta_seconds();
        // println!("{}",player_transform.translation.normalize_or_zero());
        // let forward = player_transform.rotation * player_transform.translation.normalize_or_zero();
        // player_transform.translation += forward * SHIP_SPEED * delta;

        player_transform.translation.y += speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::KeyA) {
        player_transform.translation.x -= speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::KeyS) {
        player_transform.translation.y -= speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::KeyD) {
        player_transform.translation.x += speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Space) {
        let offset_distance = 100.0;
        let object_b_position = player_transform.translation
            + player_transform.rotation * Vec3::new(offset_distance, 0.0, 0.0);

        let t =
            Transform::from_translation(object_b_position).looking_at(object_b_position, Vec3::Z);

        // let a = Transform::from_translation(
        //     player_transform.translation + -player_transform.forward() * 100.,
        // );

        commands.add(SpawnLaserCommand {
            transform: t,
            spawned_by_entity: entity,
        });
    }
}
