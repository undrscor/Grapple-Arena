use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_asset_loader::prelude::*;

mod startup;
mod player;
mod physics;
mod walls;
mod ground_detection;
mod wall_climb;
mod animation;
mod grapple;

use startup::setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            LdtkPlugin,
            RapierPhysicsPlugin::<()>::default(),
            //RapierDebugRenderPlugin::default(), //for debugging colliders
        ))

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        //implement plugins
        .add_plugins(animation::PlayerAnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(grapple::GrapplePlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(wall_climb::WallClimbPlugin)

        .run();
}