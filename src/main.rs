use std::iter::repeat_with;

use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

impl From<Direction> for Vec3 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
            Direction::UpRight => Vec3::new(1.0, 1.0, 0.0).normalize(),
            Direction::DownRight => Vec3::new(1.0, -1.0, 0.0).normalize(),
            Direction::DownLeft => Vec3::new(-1.0, -1.0, 0.0).normalize(),
            Direction::UpLeft => Vec3::new(-1.0, 1.0, 0.0).normalize(),
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..8) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::UpRight,
            5 => Direction::DownRight,
            6 => Direction::DownLeft,
            7 => Direction::UpLeft,
            _ => panic!("Random out of range 0..7"),
        }
    }
}

struct Ball {
    direction: Direction,
    speed: f32,
}

impl Ball {
    fn new() -> Self {
        let direction = repeat_with(rand::random::<Direction>)
            .take_while(|d| *d != Direction::Up && *d != Direction::Down)
            .next()
            .expect("Somehow we generated a None starter Direction for the ball");

        Ball {
            direction,
            speed: 300.0,
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_movement.system())
        .run();
}

fn ball_movement(time: Res<Time>, mut balls: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in balls.iter_mut() {
        let direction: Vec3 = ball.direction.into();
        let velocity: Vec3 = ball.speed * direction;
        let distance: Vec3 = velocity * time.delta_seconds();

        transform.translation += distance;
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
        //bottom
        .spawn(SpriteBundle {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        // Paddles
        // Player 1
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(paddle_width, paddle_height)),
            ..Default::default()
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
        //Ball
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(ball_width, ball_height)),
            ..Default::default()
        })
        .with(Ball::new());
}
