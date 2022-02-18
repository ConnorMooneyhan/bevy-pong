use crate::prelude::*;

mod ball_collision;
mod paddle_movement;
mod reset_elements;

pub fn playing_systems() -> SystemSet {
    SystemSet::on_update(GameState::Playing)
        .with_system(paddle_movement::paddle_movement_system)
        .with_system(ball_collision::ball_collision_system)
        .with_system(reset_elements::reset_elements)
}