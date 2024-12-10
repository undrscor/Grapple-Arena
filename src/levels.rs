use crate::startup::LevelMusicMap;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Assets, GlobalTransform, Handle, Query, Rect, Res, ResMut, Vec2, With};
use bevy_ecs_ldtk::{LevelIid, LevelSelection};
use bevy_ecs_ldtk::prelude::{LdtkProject, LevelMetadataAccessor};
use bevy_kira_audio::{Audio, AudioControl};
use crate::player::Player;

fn level_selection_follow_player(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
    level_music_map: Res<LevelMusicMap>, // Access the level-to-music map
    audio: Res<Audio>,
) {
    if let Ok(player_transform) = players.get_single() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("ldtk project should be loaded before player is spawned");

        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");

            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };

            // Check if the player is in the current level
            if level_bounds.contains(player_transform.translation().truncate()) {
                let current_level = LevelSelection::Iid(level_iid.clone());

                // Only proceed if the level has changed
                if *level_selection != current_level {
                    *level_selection = current_level.clone();

                    // Fetch the music handle for the new level
                    if let Some(music_handle) = level_music_map.music_map.get(level_iid.get()) {
                        // Stop the current music and play the new one
                        audio.stop();
                        audio.play(music_handle.clone()).looped().with_volume(0.8);

                        print!("Playing music for level: {}", level_iid.get());
                    } else {
                        // Log a warning if no music is found for the level
                        print!(
                            "No music found for level: {}. Default music will be played.",
                            level_iid.get()
                        );
                    }
                }
            }
        }
    }
}


pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, level_selection_follow_player);
    }
}
