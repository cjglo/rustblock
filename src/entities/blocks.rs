use bevy::prelude::*;
use crate::entities::physics::{Gravity, Collision};

pub fn spawn_grass_block(x: f32, y: f32, z: f32) -> (Gravity, Collision, SpriteBundle) {
    let bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.02, 0.75, 0.02),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(x, y, z)),
        ..default()
    };

    (Gravity{acceleration: 1.}, Collision, bundle)
}