use bevy::prelude::*;

use crate::GameSet;

#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Resource)]
struct BodySpeed(f32);

#[derive(Bundle)]
pub struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    pub fn new(position: Vec3) -> Self {
        Self {
            body: Body,
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BodySpeed(220.0))
            .add_systems(Update, move_bodies.in_set(GameSet::Movement));
    }
}

fn move_bodies(
    time: Res<Time>,
    speed: Res<BodySpeed>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    let movement_scale = speed.0 * time.delta_secs();

    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * movement_scale).extend(0.0);
    }
}
