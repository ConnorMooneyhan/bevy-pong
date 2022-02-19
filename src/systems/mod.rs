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

pub fn end_game_system(
    mut commands: Commands,
    score: Res<Score>,
    mut game_state: ResMut<State<GameState>>,
) {
    if score.is_changed() {
        if score.player_one >= GOAL_SCORE {
            game_state.set(GameState::GameOver).unwrap();
            commands.insert_resource(Winner::Player1);
        } else if score.player_two >= GOAL_SCORE {
            game_state.set(GameState::GameOver).unwrap();
            commands.insert_resource(Winner::Player2);
        }
    }
}
