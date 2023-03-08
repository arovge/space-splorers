use bevy::prelude::*;

pub mod components {
    pub use crate::components::*;
}

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Velocity(Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::default())
    }
}

impl Velocity {
    pub fn velocity(&self) -> Vec3 {
        self.0
    }
}