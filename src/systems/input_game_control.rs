use crate::prelude::*;

/// Can I seriously not think of a better name for this system?????
pub fn input_game_control_system(
    mut state: ResMut<State<GameState>>,
    mut score: ResMut<Score>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
        score.player_one = 0;
        score.player_two = 0;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        panic!("Game exited successfully.");
    }
}
