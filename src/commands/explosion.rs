use crate::{components::ExplosionTimer, resources::SpriteSheets};
use bevy::{ecs::system::Command, prelude::*};

pub const EXPLOSION_SHEET_TILE_LENGTH: usize = 16;
const EXPLOSION_DURATION: f32 = 0.8;
const EXPLOSION_IMAGE_DURATION: f32 = EXPLOSION_DURATION / EXPLOSION_SHEET_TILE_LENGTH as f32;

pub struct SpawnExplosionCommand {
    pub position: Vec3,
}

impl Command for SpawnExplosionCommand {
    fn apply(self, world: &mut World) {
        let sprite_sheets = world.resource::<SpriteSheets>();

        world.spawn((
            SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: sprite_sheets.explosion_layout_handle.clone(),
                    index: 0,
                },
                texture: sprite_sheets.explosion_texture_handle.clone(),
                transform: Transform {
                    translation: self.position,
                    ..Default::default()
                },
                ..Default::default()
            },
            ExplosionTimer(Timer::from_seconds(
                EXPLOSION_IMAGE_DURATION,
                TimerMode::Repeating,
            )),
        ));
    }
}
