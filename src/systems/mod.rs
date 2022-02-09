use crate::prelude::*;

mod ball_collision;
mod paddle_movement;

pub fn playing_systems() -> SystemSet {
    SystemSet::new()
        .with_system(paddle_movement::paddle_movement_system)
        .with_system(ball_collision::ball_collision_system)
}
