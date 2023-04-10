use bevy::prelude::*;
mod startup;
mod entities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup::startup)
        .add_system(startup::animate_sprite)
        .run();
}
