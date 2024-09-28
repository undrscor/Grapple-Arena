use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

//camera and world setup (through ldtk), could add more components
pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //camera setup
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.7;
    camera.transform.translation.x += 1024.0 / 2.0;
    camera.transform.translation.y += 512.0 / 2.0;
    commands.spawn(camera);

    //world setup
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDTK-test.ldtk"),
        ..Default::default()
    }
    );
}
