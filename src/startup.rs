use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::prelude::AudioSource;


pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
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

    // Play background music
    let music: Handle<AudioSource> = asset_server.load("DK-grapple-arena.ogg");
    info!("Loaded music: {:?}", music);
    audio.play(music).looped().with_volume((0.8)); // Play the audio in a loop
}
