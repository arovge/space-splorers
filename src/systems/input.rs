use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input);
    }
}

fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keys.any_pressed([KeyCode::KeyQ, KeyCode::Escape]) {
        app_exit_events.send(bevy::app::AppExit);
    }
}
