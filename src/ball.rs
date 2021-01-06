use bevy::{
    core::Time,
    ecs::{Query, Res},
    prelude::Transform,
    prelude::Vec3,
    sprite::{
        collide_aabb::{collide, Collision},
        Sprite,
    },
};
use std::iter::repeat_with;
use crate::direction::*;
use crate::collider::*;

pub struct Ball {
    direction: Vec3,
    speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        let direction = repeat_with(rand::random::<Direction>)
            .find(|d| *d != Direction::Up && *d != Direction::Down)
            .expect("Somehow we generated a None starter Direction for the ball");

        Ball {
            direction: direction.into(),
            speed: 300.0,
        }
    }

    pub fn collide(&mut self, collision: Collision) {
        match collision {
            Collision::Left | Collision::Right => self.direction *= Vec3::new(-1.0, 1.0, 1.0),
            Collision::Top | Collision::Bottom => self.direction *= Vec3::new(1.0, -1.0, 1.0),
        }
    }
}

pub fn ball_movement(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in query.iter_mut() {
        let direction: Vec3 = ball.direction;
        let velocity: Vec3 = ball.speed * direction;
        let distance: Vec3 = velocity * time.delta_seconds();

        transform.translation += distance;
    }
}

pub fn ball_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    colliders_query: Query<(&Collider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        for (_collider, collider_transform, collider_sprite) in colliders_query.iter() {
            if let Some(collision) = collide(
                ball_transform.translation,
                ball_sprite.size,
                collider_transform.translation,
                collider_sprite.size,
            ) {
                ball.collide(collision)
            };
        }
    }
}
