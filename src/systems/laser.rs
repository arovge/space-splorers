use crate::components::{Laser, LaserCooldown};
use bevy::prelude::*;

pub const LASER_COOLDOWN_DURATION: f32 = 0.25;
pub const LASER_SIZE: f32 = 5.;
const LASER_SPEED: f32 = 750.;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_lasers, tick_laser_cooldowns));
    }
}

fn move_lasers(time: Res<Time>, mut query: Query<(&mut Transform, &Laser), With<Laser>>) {
    for (mut transform, _laser) in query.iter_mut() {
        let delta = time.delta_seconds();
        let forward = transform.rotation * transform.translation.normalize();
        transform.translation += forward * LASER_SPEED * delta;

        // TODO: How despawn when off screen?
        // Maybe have it lose energy as it travels? Or a set distance/time to live?
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
