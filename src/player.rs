use crate::ground_detection::GroundDetection;
use crate::physics::PhysicsBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;


//player system, the parameters probably don't go here, need to figure out if they go in the bundle
// #[derive(Default, Debug, Component)]
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Component)]
pub struct Player {
    //pub facing_right: bool,
    //pub movement_speed: Velocity,
    //pub player_colliding: bool,
    //pub jump_force: f32,
    pub double_jump: bool,
}

const PLAYER_ACCELERATION_MULTIPLIER: f32 = 400.0f32; //for force multiplier
const PLAYER_TOP_SPEED: f32 = 250.0;
const PLAYER_JUMP_STRENGTH: f32 = 300.0;

//playerbundle: creates player object and assigns sprite
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,

    #[from_entity_instance]
    physics: PhysicsBundle,

    player: Player,

    #[worldly]
    worldly: Worldly, //this sets player to worldly status, meaning it persists through levels and is a child of the world itself
    ground_detection: GroundDetection,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    // #[from_entity_instance]
    // entity_instance: EntityInstance,
}

//movement system, updates player velocity but needs physics system to be finished to work properly
pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity, &GroundDetection, &mut ExternalForce, &mut GravityScale), With<Player>>,
) {
    for (mut player, mut velocity, ground_detection, mut force, mut gravity) in &mut query {
        let left = input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft);
        let right = input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight);
        let x_input = -&(left as i8) + &(right as i8);

        //implementation of forces for horizontal movement, meaning the player gradually speeds up instead of achieving max move speed instantly
        //force.force.x = (x_input as f32) * PLAYER_ACCELERATION_MULTIPLIER; //without max vel

        if right
        {
            let new_horizontal_force = calc_force_diff(
                x_input as f32,
                velocity.linvel.x,
                PLAYER_TOP_SPEED,
            );

            force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            // sprite.flip_x = false;
        } else if left
        {
            let new_horizontal_force = calc_force_diff(
                -x_input as f32,
                velocity.linvel.x,
                -PLAYER_TOP_SPEED,
            );

            force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            // sprite.flip_x = true;
        } else {
            if velocity.linvel.x.abs() > 0.01 {
                let new_horizontal_force =
                    -velocity.linvel.x;
                force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            }
        }


        //Jumping, detects if the player is on the ground so they can jump again
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground) {
            velocity.linvel.y = PLAYER_JUMP_STRENGTH; //jump height
        }
        if ground_detection.on_ground { //if the player is on the ground, it means they haven't jumped yet, so set double jump to false
            player.double_jump = false;
        }
        if input.just_pressed(KeyCode::Space) && !(player.double_jump) && !(ground_detection.on_ground) {
            velocity.linvel.y = PLAYER_JUMP_STRENGTH; //jump height
            player.double_jump = true; //since the player is not on the ground, set double jump to true so that they will only be able to jump once more before hitting the ground.
        }

        //jump higher //check numbers
        if input.pressed(KeyCode::Space) {
            if (velocity.linvel.y >= 30.) {
                gravity.0 = 60.;
            }
            else {
                gravity.0 = 100.;
            }
        }

        //fast fall
        if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
            gravity.0 = 200.;
        }

        // we can check multiple at once with `.any_*`
        if !input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS, KeyCode::Space]) {
            gravity.0 = 100.;
        }

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

    //calculates force needed to reach and maintain max speed based on input
    fn calc_force_diff(
        input: f32,
        current_velocity: f32,
        target_velocity: f32,
    ) -> f32 {
        let target_speed = target_velocity * input;
        let diff_to_make_up = target_speed - current_velocity;
        let new_force = diff_to_make_up * 2.0;
        new_force
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
