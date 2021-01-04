use bevy::{
    core::Time,
    ecs::{IntoSystem, Query, Res},
    prelude::Transform,
    prelude::{
        App, Assets, Camera2dBundle, CameraUiBundle, Color, ColorMaterial, Commands,
        DefaultPlugins, ResMut, SpriteBundle, Vec2, Vec3,
    },
    sprite::{
        collide_aabb::{collide, Collision},
        Sprite,
    },
};

mod direction;
use crate::direction::*;

use std::iter::repeat_with;
struct Ball {
    direction: Vec3,
    speed: f32,
}

struct Collider;

impl Ball {
    fn new() -> Self {
        let direction = repeat_with(rand::random::<Direction>)
            .find(|d| *d != Direction::Up && *d != Direction::Down)
            .expect("Somehow we generated a None starter Direction for the ball");

        Ball {
            direction: direction.into(),
            speed: 300.0,
        }
    }

    fn collide(&mut self, collision: Collision) {
        match collision {
            Collision::Left | Collision::Right => self.direction *= Vec3::new(-1.0, 1.0, 1.0),
            Collision::Top | Collision::Bottom => self.direction *= Vec3::new(1.0, -1.0, 1.0),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_movement.system())
        .add_system(ball_collision.system())
        .run();
}

fn ball_movement(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in query.iter_mut() {
        let direction: Vec3 = ball.direction;
        let velocity: Vec3 = ball.speed * direction;
        let distance: Vec3 = velocity * time.delta_seconds();

        transform.translation += distance;
    }
}

fn ball_collision(
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

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    let paddle_width = 20.0;
    let paddle_height = 100.0;

    let ball_width = 20.0;
    let ball_height = 20.0;

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        // Walls
        // top
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider)
        //bottom
        .spawn(SpriteBundle {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider)
        // Paddles
        // Player 1
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(paddle_width, paddle_height)),
            ..Default::default()
        })
        .with(Collider)
        //Player 2
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(
                (bounds.x / 2.0) - paddle_width,
                0.0,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(paddle_width, paddle_height)),
            ..Default::default()
        })
        .with(Collider)
        //Ball
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(ball_width, ball_height)),
            ..Default::default()
        })
        .with(Ball::new());
}
