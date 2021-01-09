use bevy::{
    ecs::IntoSystem,
    prelude::{App, DefaultPlugins},
};

mod ball;
mod collider;
mod direction;
mod levels;
mod paddle;
mod player;

use crate::ball::*;
use crate::levels::game::*;
use crate::paddle::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(game_level.system())
        .add_system(ball_movement.system())
        .add_system(ball_paddle_collision.system())
        .add_system(ball_wall_collision.system())
        .add_system(paddle_movement.system())
        .run();
}
