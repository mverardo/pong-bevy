use crate::collider::*;
use crate::direction::*;
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
use rand::seq::SliceRandom;
use rand::Rng;
use std::iter::repeat_with;

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
            speed: 500.0,
        }
    }

    fn reflect_normal(&mut self, collision: Collision) {
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

    pub fn collide_wall(&mut self, collision: Collision) {
        self.reflect_normal(collision);
    }

    pub fn collide_paddle(&mut self, collision: Collision) {
        let mut rng = rand::thread_rng();
        let normal_reflection = rng.gen_bool(0.7);

        if normal_reflection {
            self.reflect_normal(collision);
        } else {
            let v = vec![-1.0, -1.0, 0.0, 1.0, 1.0];
            let reflect_y = v.choose(&mut rng).unwrap();

            self.direction = Vec3::new(-1.0 * self.direction.x, *reflect_y, 0.0);
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

pub fn ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    colliders_query: Query<(&PaddleCollider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        for (_collider, collider_transform, collider_sprite) in colliders_query.iter() {
            if let Some(collision) = collide(
                collider_transform.translation,
                collider_sprite.size,
                ball_transform.translation,
                ball_sprite.size,
            ) {
                ball.collide_paddle(collision)
            };
        }
    }
}

pub fn ball_wall_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    colliders_query: Query<(&WallCollider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        for (_collider, collider_transform, collider_sprite) in colliders_query.iter() {
            if let Some(collision) = collide(
                collider_transform.translation,
                collider_sprite.size,
                ball_transform.translation,
                ball_sprite.size,
            ) {
                ball.collide_wall(collision)
            };
        }
    }
}
