use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod explosion;
pub mod input;
pub mod laser;
pub mod player;
pub mod ship;
pub mod ui;

pub fn cursor_position_to_world_position(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    let cursor_position = window_query.get_single().ok()?.cursor_position()?;
    let (camera, camera_transform) = camera_query.get_single().ok()?;
    camera.viewport_to_world_2d(camera_transform, cursor_position)
}
