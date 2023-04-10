use bevy::prelude::*;
use crate::entities::mobs;


#[derive(Component)]
pub struct Gravity { pub acceleration: f32 }

#[derive(Component)]
pub struct Collision;

pub fn init_gravity(query: Query<&Mobs, With<Person>>) {

}