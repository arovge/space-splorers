use bevy::prelude::*;

use crate::components::Ship;

pub fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Ship>>,
) {
    if keys.pressed(KeyCode::W) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.y += 1.;
    }

    if keys.pressed(KeyCode::A) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.x -= 1.;
    }

    if keys.pressed(KeyCode::S) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.y -= 1.;
    }

    if keys.pressed(KeyCode::D) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.x += 1.;
    }
}
