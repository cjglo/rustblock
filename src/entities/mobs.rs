use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use crate::entities::animations::AnimationTimer;
use crate::entities::animations::AnimationIndices;

pub fn spawn_player() -> (SpriteSheetBundle, AnimationIndices, AnimationTimer) {
    // Asset Settings
    let texture_handle = asset_server.load("./adventurer_sprite_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices{ first: 1, last: 6 };

    (
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    )
}