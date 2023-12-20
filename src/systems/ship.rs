use crate::components::Ship;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SPEED: f32 = 150.;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, handle_keyboard_input);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(10., 10., 10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ship,
    ));
}

fn handle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Ship>>,
) {
    if keys.pressed(KeyCode::W) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.y += SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.x -= SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::S) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.y -= SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        let mut ship_transform = query.single_mut();
        ship_transform.translation.x += SPEED * time.delta_seconds();
    }
}
