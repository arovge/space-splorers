mod commands;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use systems::{explosion::ExplosionPlugin, laser::LaserPlugin, ship::ShipPlugin, text::TextPlugin};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((ExplosionPlugin, LaserPlugin, ShipPlugin, TextPlugin))
        .run();
}
