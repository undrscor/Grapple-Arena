use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod startup;

use startup::setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LdtkPlugin,
        ))

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        .run();
}
