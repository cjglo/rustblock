use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use crate::entities::blocks;
use crate::entities::mobs;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>,
         mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Camera Settings
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 1.0)),
        },
        ..Default::default()
    });
    // World Gen
    commands.spawn(mobs::spawn_player);
    commands.spawn(blocks::spawn_grass_block(0.,-50.,0.));
}