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
    for (_laser_transform, _laser) in query.iter_mut() {
        let delta = time.delta_seconds();
        let _distance = LASER_SPEED * delta;

        // laser_transform.forward();

        // TODO: Figure out math
        // let _theta = laser.direction.y.atan2(laser.direction.x);
        // let dx = laser.direction.x * distance;
        // let dy = laser.direction.y * distance;
        // laser_transform.forward();

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
