use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use wasm_bindgen::prelude::*;

mod startup;
mod player;
mod physics;
mod walls;
mod ground_detection;
mod wall_climb;
mod animation;

use startup::setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LdtkPlugin,
            RapierPhysicsPlugin::<()>::default(),
            //RapierDebugRenderPlugin::default(), //for debugging colliders
        ))

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        //implement player plugin
        .add_plugins(animation::PlayerAnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(wall_climb::WallClimbPlugin)

        .run();
}


/*

// Website Handling
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_web() {
    console_error_panic_hook::set_once();  // This will log any panics to the browser console

    web_sys::console::log_1(&"Bevy WebAssembly - Grapple Arena - starting...".into());

    // Initialize
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy_canvas".into()),
                        ..default()
                    }),
                    ..default()
                }))

        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<()>::default())

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        //implement player plugin
        .add_plugins(animation::PlayerAnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(wall_climb::WallClimbPlugin)
        .add_plugins(grappling::GrapplingPlugin)

        .run();

    web_sys::console::log_1(&"Game initialization complete.".into());
}

 */