use std::convert::TryFrom;

use bevy::prelude::Vec3;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
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

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;

    use super::{Direction, TryFrom};

    #[test]
    fn from_vec3() {}
}
