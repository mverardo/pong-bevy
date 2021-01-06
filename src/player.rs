use bevy::prelude::KeyCode;

#[derive(Debug, PartialEq)]
pub enum PlayerCode {
    One,
    Two,
}
pub struct Player {
    pub code: PlayerCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}
