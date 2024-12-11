// player.rs
use crate::animation::*;
use crate::ground_detection::GroundDetection;
use crate::physics::PhysicsBundle;
use crate::wall_climb::ClimbDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

//use crate::startup::{setup, LevelBounds};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    player_input: PlayerInput,
    #[from_entity_instance]
    physics: PhysicsBundle,
    animation_bundle: AnimationBundle,
    movement_intent: MovementIntent,
    ground_detection: GroundDetection,
    climb_detection: ClimbDetection,
    #[worldly]
    worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Copy, Clone, Default, Debug, Component)]
pub struct Player {
    pub progression: u8,
    pub double_jumped: bool,
}

#[derive(Component, Default, Clone)]
pub struct PlayerInput {
    pub move_left: bool,
    pub move_right: bool,
    pub jump: bool,
    pub jump_held: bool,
    pub fast_fall: bool,
    pub grapple: bool,
    pub grapple_held: bool,
    pub restart: bool,
}

#[derive(Component, Default, Clone)]
pub struct MovementIntent {
    pub horizontal: f32,
    pub vertical: f32,
    pub wants_to_jump: bool,
}

const PLAYER_ACCELERATION_MULTIPLIER: f32 = 400.0f32; //for force multiplier
const PLAYER_TOP_SPEED: f32 = 250.0;
const PLAYER_JUMP_STRENGTH: f32 = 270.0;


const CAMERA_LERP_SPEED: f32 = 0.1;

pub fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    //level_bounds: Res<LevelBounds>, // Add this to access LevelBounds
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Lerp the camera position towards the player's position
            camera_transform.translation.x +=
                (player_transform.translation.x - camera_transform.translation.x)
                    * CAMERA_LERP_SPEED;
            camera_transform.translation.y +=
                (player_transform.translation.y - camera_transform.translation.y)
                    * CAMERA_LERP_SPEED;

            // Use LevelBounds to clamp the camera's position
            // let min_x = level_bounds.padding_x;
            // let max_x = level_bounds.width - level_bounds.padding_x;
            // let min_y = level_bounds.padding_y;
            // let max_y = level_bounds.height - level_bounds.padding_y;

            // camera_transform.translation.x = camera_transform.translation.x.clamp(min_x, max_x);
            // camera_transform.translation.y = camera_transform.translation.y.clamp(min_y, max_y);
        }
    }
}


pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerInput, With<Player>>,
) {
    for mut input in query.iter_mut() {
        input.move_left = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
        input.move_right = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
        input.jump = keyboard_input.just_pressed(KeyCode::Space);
        input.jump_held = keyboard_input.pressed(KeyCode::Space);
        input.fast_fall = keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);
        input.grapple = keyboard_input.just_pressed(KeyCode::KeyJ) || keyboard_input.just_pressed(KeyCode::ShiftLeft);
        input.grapple_held = keyboard_input.pressed(KeyCode::KeyJ) || keyboard_input.pressed(KeyCode::ShiftLeft);
        input.restart = keyboard_input.just_pressed(KeyCode::KeyR);
    }
}

pub fn player_movement(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut query: Query<(
        //&Abilities,
        &PlayerInput,
        &mut MovementIntent,
        &mut Player,
        &mut Velocity,
        &mut Transform,
        &GroundDetection,
        &ClimbDetection,
        &mut ExternalForce,
        &mut GravityScale,
        &mut Damping,
        &mut Sprite,
    )>,
) {
    //let walk_sound = asset_server.load("player_walk.ogg");
    for (
        //abilities,
        input,
        mut intent,
        mut player,
        mut velocity,
        mut transform,
        ground_detection,
        climb_detection,
        mut force,
        mut gravity,
        mut damping,
        mut sprite,
    ) in query.iter_mut() {
        let mut is_moving_now = false;


        //implementation of forces for horizontal movement, meaning the player gradually speeds up instead of achieving max move speed instantly
        if input.move_right
        {
            //audio.play(walk_sound.clone());
            let new_horizontal_force = calc_force_diff(
                intent.horizontal,
                velocity.linvel.x,
                PLAYER_TOP_SPEED,
            );
            force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            sprite.flip_x = false;
        } else if input.move_left
        {
            //audio.play(walk_sound.clone());
            let new_horizontal_force = calc_force_diff(
                intent.horizontal,
                velocity.linvel.x,
                -PLAYER_TOP_SPEED,
            );

            force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            sprite.flip_x = true;
        } else {
            //audio.stop();
            if velocity.linvel.x.abs() > 0.01 {
                let new_horizontal_force =
                    -velocity.linvel.x;
                force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            }
        }

        // Handle jumping
        intent.wants_to_jump = input.jump && (ground_detection.on_ground || (climb_detection.climbing && player.progression >= 2) || (!player.double_jumped && player.progression >= 1));
        if intent.wants_to_jump {
            player.double_jumped = true;
            velocity.linvel.y = PLAYER_JUMP_STRENGTH;
        }

        // Reset double jump if on ground or climbing
        if (ground_detection.on_ground && player.progression >= 1) || (climb_detection.climbing && player.progression >= 2){
            player.double_jumped = false;
        }

        // Adjust jump height
        if input.jump_held && velocity.linvel.y > 0.0 {
            gravity.0 = 55.0;
        } else {
            gravity.0 = 100.0;
        }

        // Fast fall
        if input.fast_fall {
            gravity.0 = 200.0;
        }

        // Climbing
        if climb_detection.climbing && player.progression >= 2 && !ground_detection.on_ground {
            damping.linear_damping = if input.jump_held { 0.0 } else { 15.0 };
        } else {
            damping.linear_damping = 0.0;
        }

        if input.restart && player.progression < 4 {
            //println!("playerposition: {}", transform.translation);
            *transform = reset_position(transform.clone());
            velocity.linvel = Vec2::ZERO;
            force.force = Vec2::ZERO;
        }

        // Vertical movement intent
        intent.vertical = velocity.linvel.y;
    }
}

fn calc_force_diff(input: f32, current_velocity: f32, target_velocity: f32) -> f32 {
    let target_speed = target_velocity * input.signum();
    let diff_to_make_up = target_speed - current_velocity;
    diff_to_make_up * 2.0
}


fn update_player_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut animation_assets: ResMut<AnimationAssets>,
    mut query: Query<(
        &Player,
        &mut TextureAtlas,
        &mut Handle<Image>,
        &Velocity,
        &PlayerInput, // Added to check grappling input
    )>,
) {
    for (player, mut texture_atlas, mut texture, velocity, input) in query.iter_mut() {
        // Determine animation type based on state
        let animation_type = if input.grapple_held && player.progression >= 3 {
            // If grapple is active
            AnimationType::Grapple
        } else if velocity.linvel.y.abs() > 0.1 {
            // If the player is jumping or falling
            AnimationType::Jump
        } else if velocity.linvel.x.abs() > 0.1 {
            // If the player is running
            AnimationType::Run
        } else {
            // Default to Idle animation
            AnimationType::Idle
        };

        // Fetch layout and texture for the current animation type
        let layout_handle = animation_assets.get_layout(animation_type).cloned();
        let texture_handle = animation_assets.get_texture(animation_type).cloned();

        // Ensure the layout and texture are updated for the current animation
        if let (Some(layout_handle), Some(texture_handle)) = (layout_handle, texture_handle) {
            if texture_atlas.layout != layout_handle {
                // Change animation if necessary
                texture_atlas.layout = layout_handle.clone();
                texture_atlas.index = 0; // Start at the first frame
                *texture = texture_handle.clone(); // Update the texture
                if let Some(timer) = animation_assets.get_timer_mut(animation_type) {
                    timer.reset();
                }
            }

            // Animate sprite frames if not idle
            if animation_type != AnimationType::Idle {
                if let Some(timer) = animation_assets.get_timer_mut(animation_type) {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        if let Some(layout) = texture_atlases.get(&texture_atlas.layout) {
                            let texture_count = layout.textures.len();
                            texture_atlas.index = (texture_atlas.index + 1) % texture_count;
                        }
                    }
                }
            }
        } else {
            // Log if animation assets are missing
            println!("Missing animation assets for {:?}", animation_type);
        }
    }
}

fn check_fall_death(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in player_query.iter_mut() {
        //print!("playerposition: {}", transform.translation);
        // Adjust this value based on your lowest platform/level position
        const DEATH_Y_THRESHOLD: f32 = -1257.0; // or whatever value works for your map

        if transform.translation.y < DEATH_Y_THRESHOLD {
            // Use your existing reset_position function
            *transform = reset_position(transform.clone());
            velocity.linvel = Vec2::ZERO;
        }
    }
}

pub fn reset_position(mut transform: Transform) -> Transform {
    transform.translation = Vec3::new(463.0, -344.0, 10.0);
    transform
}


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (check_fall_death,player_input, player_movement.after(player_input), update_player_animation.after(player_movement), camera_follow_system,));
    }
}

