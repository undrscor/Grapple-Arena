use bevy::prelude::*;


//camera and world setup (through ldtk), could add more components
pub(crate) fn setup(mut commands: Commands) {
    //camera setup
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.7;
    camera.transform.translation.x += 1024.0 / 2.0;
    camera.transform.translation.y += 512.0 / 2.0;
    commands.spawn(camera);

}