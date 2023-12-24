use crate::components::{Laser, LaserCooldown};
use bevy::prelude::*;

pub const LASER_SIZE: f32 = 5.;
const LASER_SPEED: f32 = 750.;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_lasers, tick_laser_cooldowns));
    }
}

fn move_lasers(time: Res<Time>, mut query: Query<&mut Transform, With<Laser>>) {
    for mut laser_query in query.iter_mut() {
        // TODO: Direction
        laser_query.translation.y += LASER_SPEED * time.delta_seconds();
        // TODO: How despawn when off screen?
    }
}

fn tick_laser_cooldowns(
    mut cooldowns: Query<(Entity, &mut LaserCooldown)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut cooldown) in &mut cooldowns {
        cooldown.0.tick(time.delta());
        if cooldown.0.finished() {
            commands.entity(entity).remove::<LaserCooldown>();
        }
    }
}
