use bevy::prelude::*;
use systems::{
    explosion::ExplosionPlugin, input::InputPlugin, laser::LaserPlugin, player::PlayerPlugin,
    ship::ShipPlugin, ui::UiPlugin,
};

mod commands;
mod components;
mod resources;
mod systems;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((
            ExplosionPlugin,
            InputPlugin,
            LaserPlugin,
            PlayerPlugin,
            ShipPlugin,
            UiPlugin,
        ))
        .run();
}
