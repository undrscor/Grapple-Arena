use crate::game_menu::{close_popup, rules_button_interaction, update_game};
use crate::game_menu::handle_loading;
use crate::game_menu::cleanup_menu;
use crate::game_menu::button_interaction;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioPlugin;

mod startup;
mod player;
mod physics;
mod walls;
mod ground_detection;
mod wall_climb;
mod animation;
mod grapple;
mod lava;
mod levels;
mod collectibles;
mod game_menu;
mod progression_ui;

use startup::setup;
use crate::player::Player;
use crate::game_menu::GameState;
use crate::game_menu::setup_menu;
use crate::progression_ui::ProgressionUiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            AudioPlugin,
            LdtkPlugin,
            RapierPhysicsPlugin::<()>::default(),
            //RapierDebugRenderPlugin::default(), //for debugging colliders
        ))
        .init_state::<GameState>() // Add the GameState

        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(GameState::MainMenu),
            setup_menu
        )
        .add_systems(
            Update,
            button_interaction.run_if(in_state(GameState::MainMenu))
        )
        .add_systems(
            Update,
            rules_button_interaction.run_if(in_state(GameState::MainMenu))
        )
        .add_systems(
            Update,
            close_popup.run_if(in_state(GameState::MainMenu))
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            cleanup_menu
        )

        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true

            },
            set_clear_color: SetClearColor::No, // Ensure we don't clear entities unexpectedly
            ..Default::default()
        })

        //implement plugins
        .add_plugins(ProgressionUiPlugin) // Add the UI plugin
        .add_plugins(animation::PlayerAnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(levels::LevelPlugin)
        .add_plugins(grapple::GrapplePlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(lava::LavaPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(wall_climb::WallClimbPlugin)
        .add_plugins(collectibles::CollectiblePlugin)

        .run();
}