use crate::prelude::*;

/// Run criteria for resetting ball and paddles
pub fn reset_elements(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<Paddle>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        paddle_query
            .iter_mut()
            .for_each(|mut transform| transform.translation.y = 0.0);
        let mut ball_transform = ball_query.single_mut();
        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = 0.0;
    }
}
