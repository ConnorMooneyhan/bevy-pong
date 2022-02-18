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

pub fn gameover_systems() -> SystemSet {
    SystemSet::on_update(GameState::GameOver)
}

pub fn end_game_system(score: Res<Score>, mut game_state: ResMut<State<GameState>>) {
    if score.is_changed() {
        if score.player_one >= 5 || score.player_two >= 5 {
            game_state.set(GameState::GameOver).unwrap();
        }
    }
}
