use bevy::prelude::*;

struct Ball {
    velocity: Vec3,
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
        let distance = ball.velocity * time.delta_seconds();

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
        .with(Ball {
            velocity: Vec3::new(300.0, 0.0, 0.0),
        });
}
