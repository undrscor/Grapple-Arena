use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Assets, GlobalTransform, Handle, Query, Rect, Res, ResMut, Vec2, With};
use bevy_ecs_ldtk::{LevelIid, LevelSelection};
use bevy_ecs_ldtk::prelude::{LdtkProject, LevelMetadataAccessor};
use crate::player::{camera_follow_system, player_input, player_movement, Player, PlayerBundle};

fn level_selection_follow_player(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
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

            if level_bounds.contains(player_transform.translation().truncate()) {
                *level_selection = LevelSelection::Iid(level_iid.clone());
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
