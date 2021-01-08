use crate::direction::*;
use crate::{collider::*, paddle::Paddle};
use bevy::{
    core::Time,
    ecs::{Query, Res},
    math::Vec2,
    prelude::Transform,
    prelude::Vec3,
    sprite::{
        collide_aabb::{collide, Collision},
        Sprite,
    },
};
use ncollide2d::{
    na::{self, Isometry2, Vector2},
    query,
    shape::Cuboid,
};
use std::iter::repeat_with;

pub struct Ball {
    pub size: Vec2,
    pub collider_shape: Cuboid<f32>, //Extract to another component
    direction: Vec3,
    speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        let direction = repeat_with(rand::random::<Direction>)
            .find(|d| *d != Direction::Up && *d != Direction::Down)
            .expect("Somehow we generated a None starter Direction for the ball");

        let size = Vec2::new(20.0, 20.0);
        Ball {
            size,
            collider_shape: Cuboid::new(Vector2::new(size.x / 2.0, size.y / 2.0)),
            direction: Direction::Right.into(), //direction.into(),
            speed: -300.0,
        }
    }

    pub fn collide(&mut self, collision: Collision) {
        match collision {
            Collision::Left => {
                if self.direction.x < 0.0 {
                    self.direction *= Vec3::new(-1.0, 1.0, 1.0)
                }
            }
            Collision::Right => {
                if self.direction.x > 0.0 {
                    self.direction *= Vec3::new(-1.0, 1.0, 1.0)
                }
            }
            Collision::Top => {
                if self.direction.y > 0.0 {
                    self.direction *= Vec3::new(1.0, -1.0, 1.0)
                }
            }
            Collision::Bottom => {
                if self.direction.y < 0.0 {
                    self.direction *= Vec3::new(1.0, -1.0, 1.0)
                }
            }
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
    colliders_query: Query<(&Collider, &Transform, &Sprite, &Paddle)>, //TODO: remove Paddle so collision works with walls
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        for (_collider, collider_transform, collider_sprite, paddle) in colliders_query.iter() {
            let ball_pos = Isometry2::new(
                Vector2::new(ball_transform.translation.x, ball_transform.translation.y),
                na::zero(),
            ); // TODO: Implement From trait to make this easier
            let collider_pos = Isometry2::new(
                Vector2::new(
                    collider_transform.translation.x,
                    collider_transform.translation.y,
                ),
                na::zero(),
            ); // TODO: Implement From trait to make this easier

            let result = query::contact(
                &ball_pos,
                &ball.collider_shape,
                &collider_pos,
                &paddle.collider_shape,
                0.0,
            );

            if let Some(contact) = result {
                dbg!(&contact.world2);
                dbg!(&contact.world2);
            }
        }
    }
}
