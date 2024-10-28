// player.rs
use crate::animation::*;
use crate::ground_detection::GroundDetection;
use crate::physics::PhysicsBundle;
use crate::wall_climb::ClimbDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    player_position: PlayerPosition,
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
    pub double_jump: bool,
}

#[derive(Clone, Default, Component)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default, Clone)]
pub struct PlayerInput {
    pub move_left: bool,
    pub move_right: bool,
    pub jump: bool,
    pub jump_held: bool,
    pub fast_fall: bool,
    pub grapple: bool,
    //pub grapple_held: bool,
    pub grapple_released: bool,
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
        //input.grapple_held = keyboard_input.pressed(KeyCode::KeyJ);
        input.grapple_released = keyboard_input.just_released(KeyCode::KeyJ);
    }
}

pub fn player_movement(
    mut query: Query<(
        &PlayerInput,
        &mut MovementIntent,
        &mut Player,
        &mut Velocity,
        &GroundDetection,
        &ClimbDetection,
        &mut ExternalForce,
        &mut GravityScale,
        &mut Damping,
        &mut Sprite,
    )>,
) {
    for (
        input,
        mut intent,
        mut player,
        mut velocity,
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
        intent.wants_to_jump = input.jump && (ground_detection.on_ground || climb_detection.climbing || !player.double_jump);
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
    mut query: Query<(&mut TextureAtlas, &mut Handle<Image>, &Velocity, &MovementIntent,)>,
) {
    for (mut texture_atlas, mut texture, velocity, intent) in query.iter_mut() {
        // Determine animation type based on velocity
        let animation_type = if velocity.linvel.y.abs() > 0.1 {
            AnimationType::Jump
        } else if velocity.linvel.x.abs() > 0.1 {
            AnimationType::Run
        } else {
            AnimationType::Idle
        };

        let layout_handle = animation_assets.get_layout(animation_type).cloned();
        let texture_handle = animation_assets.get_texture(animation_type).cloned();

        // Get the layout handle and texture handle for the current animation type
        if let (Some(layout_handle), Some(texture_handle)) = (layout_handle, texture_handle) {
            // If the animation type changed, update the texture atlas layout and texture
            if texture_atlas.layout != layout_handle {
                texture_atlas.layout = layout_handle.clone();
                texture_atlas.index = 0; // Reset to first frame of new animation
                *texture = texture_handle.clone(); // Update the texture
                if let Some(timer) = animation_assets.get_timer_mut(animation_type) {
                    timer.reset();
                }
                //println!("Animation type changed to {:?}", animation_type);
            }

            if animation_type != AnimationType::Idle {
                // Animate sprite
                if let Some(timer) = animation_assets.get_timer_mut(animation_type) {
                    timer.tick(time.delta());
                    //println!("Timer: {:?}, Finished: {}", timer, timer.just_finished());
                    if timer.just_finished() {
                        if let Some(layout) = texture_atlases.get(&texture_atlas.layout) {
                            let texture_count = layout.textures.len();
                            texture_atlas.index = (texture_atlas.index + 1) % texture_count;

                            //for custom sizing
                            // let urect = layout.textures[texture_atlas.index];
                            // sprite.rect = Some(Rect {
                            //     min: Vec2::new(urect.min.x as f32, urect.min.y as f32),
                            //     max: Vec2::new(urect.max.x as f32, urect.max.y as f32),
                            // });

                            // Ensure the sprite uses the full texture
                            //sprite.custom_size = Some(Vec2::new(urect.width() as f32, urect.height() as f32));

                            //println!("Next texture! Type: {:?}, Index: {}, Rect: {:?}", animation_type, texture_atlas.index, sprite.rect);
                        }
                    }
                } else {
                    println!("Failed to get layout or texture handle for {:?}", animation_type);
                }
            }
        }
    }
}

fn update_player_position(
    mut query: Query<(&mut PlayerPosition, &Transform), With<Player>>,
) {
    for(mut player_position, transform) in query.iter_mut() {
        //Tracks the player's position using Transform
        //.translation gives the current position of the Player
        let player_pos = transform.translation;

        player_position.x = player_pos.x;
        player_position.y = player_pos.y;
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (player_input,update_player_position.after(player_input), player_movement.after(update_player_position), update_player_animation.after(player_movement)));
    }
}

