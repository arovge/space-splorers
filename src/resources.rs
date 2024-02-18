use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteSheets {
    pub explosion: SpriteSheet,
}

pub struct SpriteSheet {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub texture_handle: Handle<Image>,
}
