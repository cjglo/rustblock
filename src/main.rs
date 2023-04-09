mod init_and_physics;

use bevy::prelude::*;
use crate::init_and_physics::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .add_system(setup::animate_sprite)
        .run();
}
