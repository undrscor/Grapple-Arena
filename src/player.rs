// player.rs
use crate::animation::*;
use crate::ground_detection::GroundDetection;
use crate::physics::PhysicsBundle;
use crate::wall_climb::ClimbDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
//use crate::startup::{setup, LevelBounds};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    //can_grapple: bool,
    player_input: PlayerInput,
    abilities: Abilities,
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
    pub double_jump: bool,
}

#[derive(Copy, Clone, Default, Component)]
pub struct Abilities {
    pub can_grapple: bool,
    pub can_wall_climb: bool,
    pub can_double_jump: bool,
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
    pub grapple_released: bool,
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
        input.grapple = keyboard_input.just_pressed(KeyCode::KeyJ);
        input.grapple_held = keyboard_input.pressed(KeyCode::KeyJ);
        input.restart = keyboard_input.just_pressed(KeyCode::KeyR);
        //input.grapple_released = keyboard_input.just_released(KeyCode::KeyJ);
    }
}

pub fn player_movement(
    mut query: Query<(
        &Abilities,
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
    for (
        abilities,
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

        //implementation of forces for horizontal movement, meaning the player gradually speeds up instead of achieving max move speed instantly
        if input.move_right
        {
            let new_horizontal_force = calc_force_diff(
                intent.horizontal,
                velocity.linvel.x,
                PLAYER_TOP_SPEED,
            );
            force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            sprite.flip_x = false;
        } else if input.move_left
        {
            let new_horizontal_force = calc_force_diff(
                intent.horizontal,
                velocity.linvel.x,
                -PLAYER_TOP_SPEED,
            );

            force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            sprite.flip_x = true;
        } else {
            if velocity.linvel.x.abs() > 0.01 {
                let new_horizontal_force =
                    -velocity.linvel.x;
                force.force.x = new_horizontal_force * PLAYER_ACCELERATION_MULTIPLIER;
            }
        }

        // Handle jumping
        intent.wants_to_jump = input.jump && (ground_detection.on_ground || climb_detection.climbing || (!player.double_jump && !abilities.can_double_jump));
        if intent.wants_to_jump {
            if !ground_detection.on_ground && !climb_detection.climbing {
                player.double_jump = true;
            }
            velocity.linvel.y = PLAYER_JUMP_STRENGTH;
        }

        // Reset double jump if on ground or climbing
        if ground_detection.on_ground || climb_detection.climbing {
            player.double_jump = false;
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
        if climb_detection.climbing {
            damping.linear_damping = if input.jump_held { 0.0 } else { 15.0 };
        } else {
            damping.linear_damping = 0.0;
        }

        if input.restart {
            println!("playerposition: {}", transform.translation);
            *transform = reset_position(transform.clone());
            velocity.linvel = Vec2::ZERO;  // Reset velocity
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
        &mut TextureAtlas,
        &mut Handle<Image>,
        &Velocity,
        &MovementIntent,
        &PlayerInput, // Added to check grappling input
    )>,
) {
    for (mut texture_atlas, mut texture, velocity, intent, input) in query.iter_mut() {
        // Determine animation type based on state
        let animation_type = if input.grapple_held {
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

pub fn reset_position(mut transform: Transform) -> Transform {
    transform.translation = Vec3::new(512.0, -344.0, 10.0);
    transform
}


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")

            .add_systems(Update, (player_input, player_movement.after(player_input), update_player_animation.after(player_movement), camera_follow_system,));
    }
}

