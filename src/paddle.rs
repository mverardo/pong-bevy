use bevy::{
    core::Time,
    ecs::{Query, Res},
    input::Input,
    prelude::Transform,
    prelude::{KeyCode, Vec3},
};
use ncollide2d::shape::Cuboid;

use crate::direction::*;
use crate::player::*;
pub struct Paddle {
    pub player: Player,
    pub speed: f32,
    pub collider_shape: Cuboid<f32>, //TODO: Extract to component
}

pub fn paddle_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        if input.pressed(paddle.player.move_up) {
            let direction: Vec3 = Direction::Up.into();
            let velocity: Vec3 = paddle.speed * direction;
            let distance: Vec3 = velocity * time.delta_seconds();

            transform.translation += distance;
        }
        if input.pressed(paddle.player.move_down) {
            let direction: Vec3 = Direction::Down.into();
            let velocity: Vec3 = paddle.speed * direction;
            let distance: Vec3 = velocity * time.delta_seconds();

            transform.translation += distance;
        }
    }
}
