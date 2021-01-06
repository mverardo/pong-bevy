use bevy::{
    core::Time,
    ecs::{IntoSystem, Query, Res},
    input::Input,
    prelude::Transform,
    prelude::{
        App, Assets, Camera2dBundle, CameraUiBundle, Color, ColorMaterial, Commands,
        DefaultPlugins, KeyCode, ResMut, SpriteBundle, Vec2, Vec3,
    },
    sprite::{
        Sprite,
    },
};

mod ball;
mod direction;
mod collider;
use crate::direction::*;
use crate::ball::*;
use crate::collider::*;

#[derive(Debug, PartialEq)]
enum PlayerCode {
    One,
    Two,
}
struct Player {
    code: PlayerCode,
    move_up: KeyCode,
    move_down: KeyCode,
}

struct Paddle {
    player: Player,
    speed: f32,
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_movement.system())
        .add_system(ball_collision.system())
        .add_system(input.system())
        .run();
}

fn input(input: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<(&Paddle, &mut Transform)>) {
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
        .with(Paddle {
            player: Player {
                code: PlayerCode::One,
                move_up: KeyCode::W,
                move_down: KeyCode::S,
            },
            speed: 300.0,
        })
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
        .with(Paddle {
            player: Player {
                code: PlayerCode::Two,
                move_up: KeyCode::Up,
                move_down: KeyCode::Down,
            },
            speed: 300.0,
        })
        //Ball
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(ball_width, ball_height)),
            ..Default::default()
        })
        .with(Ball::new());
}
