use std::ops::Add;
use super::laser::LASER_SIZE;
use crate::{
    commands::laser::SpawnLaserCommand,
    components::{Health, Laser, LaserCooldown, Ship},
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SHIP_SPEED: f32 = 150.;
const SHIP_SIZE: f32 = 10.;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                handle_keyboard_input,
                check_for_laser_hit,
                check_health_to_despawn,
            ),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(SHIP_SIZE, SHIP_SIZE, SHIP_SIZE).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ship,
        Health::default(),
    ));
}

fn handle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, Entity, &mut Ship), With<Ship>>,
    mut commands: Commands,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    cooldowns: Query<&LaserCooldown, With<Ship>>,
) {
    let (mut ship_transform, ship_entity, _ship) = query.single_mut();

    if keys.pressed(KeyCode::W) {
        ship_transform.translation.y += SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        ship_transform.translation.x -= SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::S) {
        ship_transform.translation.y -= SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        ship_transform.translation.x += SHIP_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Space) {
        let has_cooldown = cooldowns
            .get_component::<LaserCooldown>(ship_entity)
            .is_ok();

        if !has_cooldown {
            commands.add(SpawnLaserCommand {
                position: ship_transform
                    .translation
                    .clone()
                    .add(Vec3::new(0., LASER_SIZE, 0.)),
            });
            commands
                .entity(ship_entity)
                .insert(LaserCooldown(Timer::from_seconds(3., TimerMode::Once)));
        } else {
            println!("has cooldown!")
        }
    }

    if keys.pressed(KeyCode::Q) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

fn check_for_laser_hit(
    mut ship_query: Query<(&mut Health, &Transform), With<Health>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    mut commands: Commands,
) {
    let mut count = 0;
    for (laser_entity, laser_transform) in &laser_query {
        for (mut ship_health, ship_transform) in ship_query.iter_mut() {
            let is_colliding = {
                // assumes ship and laser are both spheres (they're cubes)
                // need a more complex collision system for diff shapes
                let dist = laser_transform
                    .translation
                    .xy()
                    .distance(ship_transform.translation.xy());
                dist < (SHIP_SIZE / 2.) + (LASER_SIZE / 2.)
            };
            println!("{is_colliding}");
            println!("=======");
            if is_colliding {
                commands.entity(laser_entity).despawn();
                ship_health.take_damange();
            }
        }
        count += 1;
    }
    println!("count: {count}");
}

fn check_health_to_despawn(
    ship_query: Query<(Entity, &Health), With<Ship>>,
    _commands: Commands,
) {
    for (_ship_entity, health) in &ship_query {
        if health.0 <= 0 {
            println!("you fucking died");
            // commands.entity(ship_entity).despawn();
        }
    }
}