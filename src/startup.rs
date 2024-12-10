use std::collections::HashMap;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::prelude::AudioSource;

#[derive(Resource)]
pub struct LevelMusicMap {
    pub music_map: HashMap<String, Handle<AudioSource>>,
}

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>, _audio: Res<Audio>) {
    // Spawn a zoomed-in camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..Default::default()
    });

    // Spawn the world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDTK-test.ldtk"),
        ..Default::default()
    });

    // Create a HashMap for level-to-music mapping
    let mut level_music_map = HashMap::new();
    //Level_1
    level_music_map.insert(
        "78137f20-9b00-11ef-85d2-918c41126c86".to_string(),
        asset_server.load("grapple-Arena-dnb.ogg"),
    );
    //Level_2
    level_music_map.insert(
        "89f13410-9b00-11ef-85d2-030c58c7d34b".to_string(),
        asset_server.load("grapple-Arena-peaceful-sticatto.ogg"),
    );
    //Level_3
    level_music_map.insert(
        "aa737fe0-9b00-11ef-85d2-cd1f6eb084b1".to_string(),
        asset_server.load("CHRONO-grapple-arena.ogg"),
    );
    //Level_4
    level_music_map.insert(
        "f2f338f0-9b00-11ef-85d2-151712402bd4".to_string(),
        asset_server.load("DK-grapple-arena.ogg"),
    );
    //Level_0
    level_music_map.insert(
        "69cafc60-4ce0-11ef-ac02-af3d88f88f16".to_string(),
        asset_server.load("grapple.ogg"),
    );

    // Wrap the HashMap in LevelMusicMap and insert it as a resource
    commands.insert_resource(LevelMusicMap { music_map: level_music_map });

    // Play initial background music (optional)
    //audio.play(asset_server.load("grapple.ogg")).looped().with_volume(0.8);
}
    //
    // // Play background music
    // let music: Handle<AudioSource> = asset_server.load("DK-grapple-arena.ogg");
    // info!("Loaded music: {:?}", music);
    // audio.play(music).looped().with_volume((0.8)); // Play the audio in a loop
//}
