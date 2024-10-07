use crate::player::{Player, PlayerInput};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::default::Default;


#[derive(Clone, Bundle, Default)]
pub struct GrappleBundle {
    grapple: Grapple,
    state: HookState,
    sprite: SpriteBundle,
    rigid_body: RigidBody,
    //dominance_group: Dominance,
    velocity: Velocity,
    rotation_constraints: LockedAxes,
    collider: Collider,
    collider_groups: CollisionGroups,
    joint: Joint
}

#[derive(Clone, Component, Copy, Debug, Hash, Default)]
pub enum HookState {
    #[default]
    // pub speed: f32,
    // pub origin: Vec2,
    // pub max_distance: f32,
    Shooting,
    Latched,
}

#[derive(Copy, Clone, Default, Debug, Component)]
pub struct Grapple {
    // pub speed: f32,
    // pub origin: Vec2,
    // pub max_distance: f32,
}

pub fn spawn_grapple(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    spawn_location: &Transform,
    spawn_direction: &bool,
) {
    let xdir = if *spawn_direction { 1.0 } else { -1.0 };
    commands.spawn(GrappleBundle {
        grapple: Grapple {},
        state: HookState::Shooting,
        sprite: SpriteBundle {
            texture: asset_server.load("hook.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..Default::default()
            },
            transform: *spawn_location,
            ..Default::default()
        },
        rigid_body: RigidBody::Dynamic,
        //dominance_group: Dominance::group(-1),
        collider_groups: CollisionGroups::new((Group::GROUP_3), (Group::GROUP_2)),
        velocity: Velocity {
            linvel: Vec2::new(xdir * 200.0, 200.0),
            angvel: 0.0,
        },
        rotation_constraints: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y,
        collider: Collider::cuboid(7., 7.),
    });
}

pub fn launch_grapple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&Transform, &PlayerInput, &Sprite), With<Player>>,
) {
    for (transform, input, player_sprite) in player_query.iter() {
        if input.grapple {
            println!("spawn grapple");
            spawn_grapple(&mut commands, &asset_server, &transform, &!player_sprite.flip_x);
        }
    }
}

pub fn latch_grapple(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,

) {

}

pub struct GrapplePlugin;
impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, launch_grapple);
    }
}

