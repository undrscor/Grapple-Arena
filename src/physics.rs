//physics.rs
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

//physics system implementation, documentation at https://rapier.rs/docs/
#[derive(Default, Bundle, Clone)]
pub struct PhysicsBundle {
    //assigns rigid body component: https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
    pub rigid_body: RigidBody,
    //adds collider, velocity, locked rotation option, gravity, friction
    //pub mass: MassProperties,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub velocity: Velocity,
    pub force: ExternalForce,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub linear_damping: Damping,
    //pub joint: RevoluteJoint,
}

//implements physics bundle, using "from" conversion for different entities
impl From<&EntityInstance> for PhysicsBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => PhysicsBundle {
                collider: Collider::cuboid(10., 16.),
                collision_groups: CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.3,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                gravity_scale: GravityScale(100.0),
                linear_damping: Damping{
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                },
                // joint: RevoluteJoint {
                //     data: Default::default(),
                // },
                ..Default::default()
            },
            "Grapple" => PhysicsBundle {
                collider: Collider::ball(2.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y,
                friction: Friction::new(0.0),
                gravity_scale: GravityScale(0.0),
                ..Default::default()
            },
            _ => PhysicsBundle::default(),
        }
    }
}

//sensor system (occurs when objects overlap, could use later)
// #[derive(Default, Bundle)]
// pub struct SensorBundle {
//     pub collider: Collider,
//     pub sensor: Sensor,
//     pub active_events: ActiveEvents,
//     pub rotation_constraints: LockedAxes,
// }
//
// impl From<IntGridCell> for SensorBundle {
//     fn from(int_grid_cell: IntGridCell) -> SensorBundle {
//         let rotation_constraints = LockedAxes::ROTATION_LOCKED;
//
//         SensorBundle::default()
//     }
// }
