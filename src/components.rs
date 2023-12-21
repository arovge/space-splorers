use bevy::prelude::*;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct LaserCooldown(pub Timer);

#[derive(Component)]
pub struct Health(pub i32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

impl Health {
    pub fn take_damange(&mut self) {
        self.0 -= 25;
    }
}

#[derive(Component)]
pub struct CoordinatesText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct LaserCooldownText;

#[derive(Component)]
pub struct Laser {
    pub direction: Vec3,
}
