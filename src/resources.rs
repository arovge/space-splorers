use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteSheets {
    pub explosion_layout_handle: Handle<TextureAtlasLayout>,
    pub explosion_texture_handle: Handle<Image>,
}
