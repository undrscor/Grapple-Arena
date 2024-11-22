use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


// #[derive(Resource)]
// pub struct LevelBounds {
//     pub width: f32,
//     pub height: f32,
//     pub padding_x: f32,
//     pub padding_y: f32,
// }
pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Insert LevelBounds resource
    // commands.insert_resource(LevelBounds {
    //     width: 1024.0, // Default values
    //     height: 512.0,
    //     padding_x: 250.0,
    //     padding_y: 200.0,
    // });

    // Spawn a zoomed-in camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5, // Adjust zoom level
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 999.9), // Set z to render above everything else
        ..Default::default()
    });

    // Spawn the world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDTK-test.ldtk"),
        ..Default::default()
    });

}

