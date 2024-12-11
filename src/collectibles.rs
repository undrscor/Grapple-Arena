use bevy::prelude::*;
use bevy::sprite::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::physics::{SensorBundle};
use crate::player::Player;
use bevy_kira_audio::{Audio, AudioControl};


#[derive(Clone, Bundle, Default, LdtkEntity)]
pub struct CollectibleBundle{
    collectible: Collectible,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    pub physics: SensorBundle,
    #[worldly]
    pub worldly: Worldly
}

#[derive(Clone, Component, Default)]
pub struct Collectible{}

#[derive(Clone, Component)]
pub struct FadeOutText {
    timer: Timer,
}

impl Default for FadeOutText {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

// System to handle the fade out
fn handle_text_fade(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Text, &mut FadeOutText)>,
) {
    for (entity, mut text, mut fade) in query.iter_mut() {
        fade.timer.tick(time.delta());
        // Update text alpha
        text.sections[0].style.color.set_alpha(fade.timer.fraction_remaining());
        // Remove when done
        if fade.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}


fn collect_collectible(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    mut player_query: Query<(Entity, &mut Player, &mut Transform, &mut Velocity), With<Player>>,
    collectible_query: Query<Entity, With<Collectible>>,
) {
    let (player_entity, mut player, mut player_transform, mut player_velocity) = if let Ok(player) = player_query.get_single_mut() {
        player
    } else {
        return;
    };

    for collectible_entity in collectible_query.iter() {
        let collected = asset_server.load("sounds/collect.ogg");
        if rapier_context.intersection_pair(player_entity, collectible_entity) == Some(true)
        {
            //print!("collected collectible");
            commands.entity(collectible_entity).despawn();
            player.progression += 1;
            audio.play(collected.clone());

            let mut find_text = "";

            match player.progression {
                4 => {
                    find_text = "The scientists have deemed you too \n valuable for a test subject. \n\nYou will remain in containment indefinitely \n Trial complete.";
                }
                3 => {
                    find_text = "You unlocked the Grappling Hook!\nPress J to grapple!";
                }
                2 => {
                    find_text = "You unlocked wall climbing!\nHug walls to cling and jump off!";
                }
                1 => {
                    find_text = "You unlocked Double Jump!\nJump midair to gain extra height!";
                }
                _ => {}
            }

            if (player.progression) <= 3 {
                print!("player progression: {}", player.progression);
                commands.entity(player_entity).with_children(|parent| {
                    parent.spawn((
                            Text2dBundle {
                            text: Text::from_section(
                                find_text,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ).with_justify(JustifyText::Center),
                            text_anchor: Anchor::Center,
                            transform: Transform::from_xyz(0.0, 70.0, 5.0),
                            ..default()
                        },
                        FadeOutText::default()
                    ));
                });
            } else if player.progression >= 4 {
                player_transform.translation = Vec3::new(463.0, -1072.0, 10.0);
                player_velocity.linvel = Vec2::ZERO;
                commands.spawn(
                    Text2dBundle {
                        text: Text::from_section(
                            find_text,
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 10.0,
                                color: Color::WHITE,
                            },
                        ).with_justify(JustifyText::Center),
                        text_anchor: Anchor::Center,
                        transform: Transform::from_xyz(463.0, -1012.0, 5.0),
                        ..default()
                    }
                );
            }
        }
    }
}


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
            app.add_systems(Update, (animate_collectibles, collect_collectible, handle_text_fade))
                .register_ldtk_entity::<CollectibleBundle>("DefaultCollectible");
            //.register_ldtk_entity::<CollectibleBundle>("Boots");
            //.register_ldtk_entity::<CollectibleBundle>("Pills");
            //.register_ldtk_entity::<CollectibleBundle>("Hook");
            //.register_ldtk_entity::<CollectibleBundle>("Coin");
            //.register_ldtk_entity::<CollectibleBundle>("DefaultCollectible");

        }
    }
