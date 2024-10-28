//grapple.rs
use crate::ground_detection::GroundDetection;
use crate::player::{Player, PlayerInput};
use crate::wall_climb::ClimbDetection;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::default::Default;

#[derive(Clone, Default, Bundle)]
pub struct GrappleBundle {
    grapple: Grapple,
    state: HookState,
    sprite: SpriteBundle,
    rigid_body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    collider_groups: CollisionGroups,
    rotation_constraints: LockedAxes,
    active_events: ActiveEvents,
    active_collision_types: ActiveCollisionTypes,
}

#[derive(Copy, Clone, Default, Debug, Component)]
pub struct Grapple {
    flying_direction: f32,
}

#[derive(Clone, Component, Copy, Debug, Hash, Default)]
#[derive(PartialEq)]
pub enum HookState {
    #[default]
    Shooting,
    Latched,
    Swinging,
}

pub fn grapple_launch(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent_query: Query<(&Transform, &Velocity, &Sprite, &PlayerInput), (With<Player>)>,
) {
    for (player_transform, player_velocity, player_sprite, input) in parent_query.iter() {
        let direction = if !player_sprite.flip_x { 1.0 } else { -1.0 };
        let additional_velocity = Vec2::new(300.0 * direction, 300.0);
        if input.grapple {
            commands.spawn(
                GrappleBundle {
                    grapple: Grapple { flying_direction: direction },
                    state: Default::default(),
                    sprite: SpriteBundle {
                        texture: asset_server.load("hook.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(12.0, 12.0)),
                            flip_x: player_sprite.flip_x,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(player_transform.translation.x + (10. * direction), player_transform.translation.y + 10., 0.0),
                        ..Default::default()
                    },
                    rigid_body: RigidBody::KinematicVelocityBased,
                    velocity: Velocity::linear(player_velocity.linvel + additional_velocity),
                    collider: Collider::ball(3.0),
                    collider_groups: CollisionGroups::new(Group::GROUP_3, Group::GROUP_2),
                    rotation_constraints: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y,
                    active_events: ActiveEvents::COLLISION_EVENTS,
                    active_collision_types: ActiveCollisionTypes::KINEMATIC_STATIC,
                }
            );
        }
    }
}

pub fn update_grapple(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut grapple_query: Query<(Entity, &Grapple, &Transform, &mut HookState, &mut RigidBody, &mut Velocity,), With<Grapple>>,
    mut collision_event: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Player, &Transform, &mut Velocity, &PlayerInput, &GroundDetection, &ClimbDetection), (With<Player>, Without<Grapple>)>,
) {
    for (grapple_entity, grapple, grapple_position, mut state, mut rigidbody, mut grapple_velocity, ) in grapple_query.iter_mut() {
        for (player_entity, mut player, player_position, mut player_velocity, player_input, ground_detection, climb_detection) in player_query.iter_mut() {
            match *state {
                HookState::Shooting => {
                    gizmos.line_2d(
                        Vec2::new(grapple_position.translation.x, grapple_position.translation.y),
                        Vec2::new(player_position.translation.x, player_position.translation.y),
                        Color::BLACK,
                    );
                    grapple_velocity.linvel.x = player_velocity.linvel.x + 700. * grapple.flying_direction;
                    grapple_velocity.linvel.y = 700.;
                    // Check for collisions
                    for collision_event in collision_event.read() {
                        match collision_event {
                            CollisionEvent::Started(e1, e2, _) => {
                                if grapple_entity == *e1 {
                                    *state = HookState::Latched;
                                } else if grapple_entity == *e2 {
                                    *state = HookState::Latched;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                HookState::Latched => {
                    *rigidbody = RigidBody::Fixed;
                    let distance = Vec2::distance(Vec2::new(grapple_position.translation.x, grapple_position.translation.y), Vec2::new(player_position.translation.x, player_position.translation.y));
                    let joint = RopeJointBuilder::new(Vec2::new(grapple_position.translation.x - player_position.translation.x, grapple_position.translation.y - player_position.translation.y).length())
                        .local_anchor1(Vec2::ZERO)
                        .local_anchor2(Vec2::ZERO);

                    commands.entity(player_entity).insert(
                        ImpulseJoint::new(grapple_entity, *joint.build().set_contacts_enabled(false).set_max_distance(distance)),
                    );
                    player.double_jump = false;
                    if !ground_detection.on_ground {
                        player_velocity.linvel.y = -300.0;
                    }
                    if player_velocity.linvel.x.abs() < 100. {
                        player_velocity.linvel.x = 100.0 * grapple.flying_direction;
                    }
                    *state = HookState::Swinging;
                }
                HookState::Swinging => {
                    *rigidbody = RigidBody::Fixed;
                    player_velocity.linvel.x = player_velocity.linvel.x + 5. * grapple.flying_direction;
                    gizmos.line_2d(
                        Vec2::new(grapple_position.translation.x, grapple_position.translation.y),
                        Vec2::new(player_position.translation.x, player_position.translation.y),
                        Color::BLACK,
                    );
                }
            }
            if player_input.grapple_released || player_input.jump || (state.eq(&HookState::Swinging) && (climb_detection.climbing)) {
                commands.entity(player_entity).remove::<ImpulseJoint>();
                commands.entity(grapple_entity).despawn();
            }
        }
    }
}


pub struct GrapplePlugin;
impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grapple_launch);
        app.add_systems(Update, update_grapple);
    }
}

