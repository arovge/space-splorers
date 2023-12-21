mod commands;
mod components;
mod systems;

use bevy::prelude::*;
use systems::{laser::LaserPlugin, ship::ShipPlugin, text::TextPlugin};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((LaserPlugin, ShipPlugin, TextPlugin))
        .run();
}
