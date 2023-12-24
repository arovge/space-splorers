use bevy::prelude::*;
use crate::{commands::explosion::EXPLOSION_SHEET_TILE_LEN, components::ExplosionTimer};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_explosions);
    }
}

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<ExplosionTimer>>,
) {
    for (entity, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite tile
            if sprite.index >= EXPLOSION_SHEET_TILE_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}
