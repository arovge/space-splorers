use crate::{
    commands::explosion::EXPLOSION_SHEET_TILE_LENGTH,
    components::ExplosionTimer,
    resources::{SpriteSheet, SpriteSheets},
};
use bevy::prelude::*;

const EXPLOSION_SHEET: &str = "explosion_sheet.png";
const EXPLOSION_TILE_SHEET_ROWS: usize = 4;
const EXPLOSION_TILE_SHEET_COLS: usize = 4;
const EXPLOSION_TILE_SIZE: Vec2 = Vec2::new(64., 64.);

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_explosions);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let layout = TextureAtlasLayout::from_grid(
        EXPLOSION_TILE_SIZE,
        EXPLOSION_TILE_SHEET_COLS,
        EXPLOSION_TILE_SHEET_ROWS,
        None,
        None,
    );
    let layout_handle = atlases.add(layout);
    let sprite_sheets = SpriteSheets {
        explosion: SpriteSheet {
            layout_handle,
            texture_handle,
        },
    };
    commands.insert_resource(sprite_sheets);
}

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlas), With<ExplosionTimer>>,
) {
    for (entity, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            // move to next sprite tile
            sprite.index += 1;

            if sprite.index >= EXPLOSION_SHEET_TILE_LENGTH {
                commands.entity(entity).despawn();
            }
        }
    }
}
