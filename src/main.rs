mod commands;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use commands::explosion::EXPLOSION_SHEET;
use resources::SpriteSheets;
use systems::{explosion::ExplosionPlugin, laser::LaserPlugin, ship::ShipPlugin, text::TextPlugin};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
    let explosion_tiles = texture_atlases.add(texture_atlas);
    let sprite_sheets = SpriteSheets { explosion_tiles };
    commands.insert_resource(sprite_sheets);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((ExplosionPlugin, LaserPlugin, ShipPlugin, TextPlugin))
        .run();
}
