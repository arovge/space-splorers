mod components;
mod systems;

use bevy::prelude::*;
use systems::{ship::ShipPlugin, text::TextPlugin};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((ShipPlugin, TextPlugin))
        .run();
}
