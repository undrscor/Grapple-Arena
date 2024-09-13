use crate::physics::PhysicsBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;


//player system, the parameters probably don't go here, need to figure out if they go in the bundle
// #[derive(Default, Debug, Component)]
#[derive(Copy, Clone, Eq, PartialEq,Default, Debug, Component)]
pub struct Player {
    //pub facing_right: bool,
    //pub movement_speed: Velocity,
    //pub player_colliding: bool,
    //pub jump_force: f32,
}

pub const PLAYER_SPEED_MULTIPLIER: i8 = 100; //maybe take this value from player movespeed component

//playerbundle: creates player object and assigns sprite, todo add more components(?), implement physics
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,

    #[from_entity_instance]
    physics: PhysicsBundle,

    player: Player,

    #[worldly]
    worldly: Worldly, //this sets player to worldly status, meaning it persists through levels and is a child of the world itself

    // The whole EntityInstance can be stored directly as an EntityInstance component
    // #[from_entity_instance]
    // entity_instance: EntityInstance,

}

//movement system, updates player velocity but needs physics system to be finished to work properly
pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    //query request cant seem to find correct player object(?)
    mut query: Query<(&mut Player, &mut Velocity), With<Player>>,
) {
    for (mut player, mut velocity) in &mut query {
        let left = input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft);
        let right = input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight);
        let x_input = -(left as i8) + right as i8;

        velocity.linvel.x = (x_input * PLAYER_SPEED_MULTIPLIER) as f32;

        //system to turn the player towards the direction of movement(needs more implementation)
        // if right {
        //     player.facing_right = true;
        //     //print!("{velocity:?}");
        // }
        // if left {
        //     player.facing_right = false;
        //     //print!("{velocity:?}");
        // }
    }
}

//to test if entity is found
pub(crate) fn reader(query: Query<&Player, With<Player>>) {
    if let Ok(player) = query.get_single() {
        println!("found player: {player:?}");
    } else {
        println!("not found")
    }
}

//to test if entity is changed
pub(crate) fn react_to_player_changing(
    query: Query<Ref<Player>>
) {
    for player in &query {
        if player.is_changed() {
            println!("player changed!!!");
        }
    }
}

//this can turn velocity into transform manually/modify it(?)
// pub(crate) fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
//     for (mut transform, velocity) in &mut query {
//         transform.translation.x += velocity.x * time.delta_seconds();
//         transform.translation.y += velocity.y * time.delta_seconds();
//     }
// }

//player plugin to register player and add movement system
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
