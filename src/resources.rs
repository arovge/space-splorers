use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteSheets {
    pub explosion_tiles: Handle<TextureAtlas>,
}
