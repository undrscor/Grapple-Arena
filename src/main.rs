use bevy::prelude::*;


mod startup;

use startup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

