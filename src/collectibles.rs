use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::physics::{PhysicsBundle, SensorBundle};
use crate::player::Player;
use bevy_kira_audio::{Audio, AudioControl};


#[derive(Clone, Bundle, Default, LdtkEntity)]
pub struct CollectibleBundle{
    collectible: Collectible,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    pub physics: SensorBundle,
    // pub collider: Collider,
    // pub sensor: Sensor,
    #[worldly]
    pub worldly: Worldly
}

#[derive(Clone, Component, Default)]
pub struct Collectible{

}

fn collect_collectible(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(Entity, &mut Player, &mut Transform, &mut Velocity), With<Player>>,
    collectible_query: Query<(Entity), With<Collectible>>, asset_server: Res<AssetServer>,  audio: Res<Audio>,
) {
    let collected = asset_server.load("collect.ogg");

    let (player_entity, mut player, mut player_transform, mut player_velocity) = if let Ok(player) = player_query.get_single_mut() {
        player
    } else {
        return;
    };

    for (collectible_entity) in collectible_query.iter() {
        if rapier_context.intersection_pair(player_entity, collectible_entity) == Some(true)
        {
            //print!("collected collectible");
            commands.entity(collectible_entity).despawn();
            player.progression += 1;
            audio.play(collected.clone());

            if player.progression >= 4 {
                player_transform.translation = Vec3::new(512.0, -344.0, 10.0);
                player_velocity.linvel = Vec2::ZERO;

            }
        }
    }
}

// fn complete_game(
//     mut commands: Commands,
//     mut player_query: Query<(Entity, &Player, &mut Transform, &mut Velocity), With<Player>>,
// ) {
//     for(player_entity, player, mut player_transform, mut player_velocity) in player_query.iter_mut() {
//         if player.progression >= 4 {
//             player_transform.translation = Vec3::new(512.0, -344.0, 10.0);
//             player_velocity.linvel = Vec2::ZERO;
//         }
//     }
// }

fn animate_collectibles(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Collectible>>,
) {
    for mut transform in query.iter_mut() {
        // Add a floating animation
        transform.translation.y += (time.elapsed_seconds() * 2.0).sin() * 0.15;
        transform.rotation = Quat::from_rotation_z(-time.elapsed_seconds() * 0.6);
    }
}

pub struct CollectiblePlugin;
impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_collectibles, collect_collectible))
            .register_ldtk_entity::<CollectibleBundle>("DefaultCollectible");
            //.register_ldtk_entity::<CollectibleBundle>("Boots");
            //.register_ldtk_entity::<CollectibleBundle>("Pills");
            //.register_ldtk_entity::<CollectibleBundle>("Hook");
            //.register_ldtk_entity::<CollectibleBundle>("Coin");
            //.register_ldtk_entity::<CollectibleBundle>("DefaultCollectible");

    }
}