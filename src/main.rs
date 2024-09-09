use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod startup;
mod player;

use startup::setup;
use crate::player::{react_to_player_changing, reader, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LdtkPlugin,
        ))

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        //implement player plugin
        .add_plugins(PlayerPlugin)

        //player tests:
        .add_systems(Update, player::reader)
        //.add_systems(Update, player::react_to_player_changing)

        .run();
}
