use bevy::{
    prelude::Transform,
    prelude::{
        Assets, Camera2dBundle, CameraUiBundle, Color, ColorMaterial, Commands, KeyCode, ResMut,
        SpriteBundle, Vec2, Vec3,
    },
    sprite::Sprite,
};

use crate::ball::*;
use crate::collider::*;
use crate::paddle::*;
use crate::player::*;

pub fn game_level(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let wall_thickness = 30.0;
    let bounds = Vec2::new(900.0, 600.0);

    let paddle_width = 30.0;
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
        .with(WallCollider)
        //bottom
        .spawn(SpriteBundle {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(WallCollider)
        // Paddles
        // Player 1
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(paddle_width, paddle_height)),
            ..Default::default()
        })
        .with(PaddleCollider)
        .with(Paddle {
            player: Player {
                code: PlayerCode::One,
                move_up: KeyCode::W,
                move_down: KeyCode::S,
            },
            speed: 600.0,
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
        .with(PaddleCollider)
        .with(Paddle {
            player: Player {
                code: PlayerCode::Two,
                move_up: KeyCode::Up,
                move_down: KeyCode::Down,
            },
            speed: 600.0,
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
