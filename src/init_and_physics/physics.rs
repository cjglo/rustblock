use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity { acceleration: f32 }

pub struct Mob(u64);

pub fn spawn_grass_block(x: f32, y: f32, z: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.02, 0.75, 0.02),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(x, y, z)),
        ..default()
    }
}