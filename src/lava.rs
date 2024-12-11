use crate::physics::SensorBundle;
use crate::player::reset_position;
use crate::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Lava;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct LavaBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub lava: Lava,
}

// Component to track lava contact time(triggers after 0.6 seconds by default implementation)
#[derive(Component)]
struct LavaContact {
    timer: Timer,
}

impl Default for LavaContact {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.6, TimerMode::Once),
        }
    }
}

// Add this component to track flashing
#[derive(Component)]
struct BurningEffect {
    original_color: Color,
    timer: Timer,
}

impl Default for BurningEffect {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            original_color: Color::WHITE,
        }
    }
}

fn detect_lava(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    lava_query: Query<Entity, With<Lava>>,
    mut player_query: Query<(Entity, &mut Sprite), With<Player>>,
    burning_query: Query<&BurningEffect>,
) {
    let (player_entity, mut sprite) = if let Ok(player) = player_query.get_single_mut() {
        player
    } else {
        return;
    };

    // Check if player is overlapping with any lava
    let mut is_in_lava = false;
    for lava_entity in lava_query.iter() {
        if rapier_context.intersection_pair(player_entity, lava_entity) == Some(true) {
            is_in_lava = true;
            break;
        }
    }

    // Handle entering/leaving lava
    let is_burning = burning_query.contains(player_entity);

    if is_in_lava && !is_burning {
        // Player just entered lava
        let burning_effect = BurningEffect {
            original_color: sprite.color,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        };
        commands.entity(player_entity)
            .insert(LavaContact::default())
            .insert(burning_effect);
    } else if !is_in_lava && is_burning {
        // Player just left lava
        if let Ok(_burning) = burning_query.get(player_entity) {
            sprite.color = Color::WHITE;
            commands.entity(player_entity)
                .remove::<LavaContact>()
                .remove::<BurningEffect>();
        }
    }
}


fn check_lava_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &mut Velocity, &mut LavaContact, &mut Sprite, Option<&BurningEffect>), With<Player>>,
) {
    for (entity, mut transform, mut velocity, mut contact, mut sprite, burning_effect) in player_query.iter_mut() {
        contact.timer.tick(time.delta());

        if contact.timer.just_finished() {
            // Restore original color before resetting position
            if let Some(burning) = burning_effect {
                sprite.color = burning.original_color;
            }

            // Respawn after 1 second
            *transform = reset_position(transform.clone());
            velocity.linvel = Vec2::ZERO;

            // Remove the contact timer
            commands.entity(entity).remove::<LavaContact>().remove::<BurningEffect>();
        }
    }
}

fn burning_effect(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut BurningEffect, Has<LavaContact>)>,
) {
    for (mut sprite, mut effect, has_contact) in query.iter_mut() {
        if !has_contact {
            sprite.color = effect.original_color;
            continue;
        }
        effect.timer.tick(time.delta());
        if effect.timer.just_finished() {
            // Toggle between red and original color
            sprite.color = if sprite.color == Color::srgb(1.0, 0.0, 0.0) {
                effect.original_color
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            };
        }
    }
}

pub struct LavaPlugin;

impl Plugin for LavaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            detect_lava,
            check_lava_timer.after(detect_lava),
            burning_effect,
        )).register_ldtk_int_cell::<LavaBundle>(2);
    }
}
