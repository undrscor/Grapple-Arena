use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;


#[derive(Component)]
pub struct ClimbSensor {
    pub climb_detection_entity: Entity,
    pub intersecting_climbables: HashSet<Entity>,
}

#[derive(Clone, Default, Component)]
pub struct ClimbDetection {
    pub climbing: bool,
}

pub fn spawn_climb_sensor(
    mut commands: Commands,
    detect_climb: Query<(Entity, &Collider), Added<ClimbDetection>>,
) {
    for (entity, shape) in &detect_climb {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {
                x: half_extents_x,
                //y: half_extents_y,
                ..
            } = cuboid.half_extents();

            //let detector_shape = Collider::cuboid(half_extents_x / 2.0, 2.);
            //let sensor_translation = Vec3::new(0., -half_extents_y, 0.);
            let detector_shape = Collider::cuboid(half_extents_x/1.5, 1.0);
            //let sensor_translation = Vec3::new(0., 0., 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    //.insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(ClimbSensor {
                        climb_detection_entity: entity,
                        intersecting_climbables: HashSet::new(),
                    });
            });
        }
    }
}

pub fn climb_detection(
    mut climb_sensors: Query<&mut ClimbSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>,
) {
    for collision_event in collisions.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                //println!("Climbin!");
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = climb_sensors.get_mut(*e2) {
                        sensor.intersecting_climbables.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = climb_sensors.get_mut(*e1) {
                        sensor.intersecting_climbables.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = climb_sensors.get_mut(*e2) {
                        sensor.intersecting_climbables.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = climb_sensors.get_mut(*e1) {
                        sensor.intersecting_climbables.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_climbing(
    mut climb_detectors: Query<&mut crate::wall_climb::ClimbDetection>,
    climb_sensors: Query<&crate::wall_climb::ClimbSensor, Changed<crate::wall_climb::ClimbSensor>>,
) {
    for sensor in &climb_sensors {
        if let Ok(mut climb_detection) = climb_detectors.get_mut(sensor.climb_detection_entity) {
            climb_detection.climbing = !sensor.intersecting_climbables.is_empty();
        }
    }
}

pub struct WallClimbPlugin;
impl Plugin for crate::wall_climb::WallClimbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, crate::wall_climb::spawn_climb_sensor)
            .add_systems(Update, crate::wall_climb::climb_detection)
            .add_systems(Update, crate::wall_climb::update_climbing);
    }
}
