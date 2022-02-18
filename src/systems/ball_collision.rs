use crate::prelude::*;

// QUERY FOR TEXTSECTION AND EXTRACT VALUE?????
pub fn ball_collision_system(
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut score: ResMut<Score>,
    mut left_scoreboard_text_query: Query<&mut Text, With<LeftScoreboard>>,
    mut right_scoreboard_text_query: Query<
        &mut Text,
        (With<RightScoreboard>, Without<LeftScoreboard>),
    >,
) {
    let mut left_scoreboard_text = left_scoreboard_text_query.single_mut();
    let mut right_scoreboard_text = right_scoreboard_text_query.single_mut();
    let (mut ball, mut ball_transform) = ball_query.single_mut();
    let mut x_multiplier = 1.0;
    let mut y_multiplier = 1.0;

    for paddle_transform in paddle_query.iter() {
        // Collide with paddle
        let collision = collide(
            ball_transform.translation,
            ball_transform.scale.truncate(),
            paddle_transform.translation,
            paddle_transform.scale.truncate(),
        );

        // Reflect off paddle
        if let Some(collision) = collision {
            match collision {
                Collision::Left => x_multiplier = -1.0,
                Collision::Top => y_multiplier = -1.0,
                Collision::Right => x_multiplier = -1.0,
                Collision::Bottom => y_multiplier = -1.0,
            };
        }
    }

    // Reflect off top edge
    if ball_transform.translation.y > TOP - BALL_RADIUS {
        y_multiplier = -1.0;
    }

    // Reflect off bottom edge
    if ball_transform.translation.y < BOTTOM + BALL_RADIUS {
        y_multiplier = -1.0;
    }

    // Score on right collision
    if ball_transform.translation.x > RIGHT - BALL_RADIUS {
        score.player_one += 1;
        left_scoreboard_text.sections[0].value = score.player_one.to_string();
        x_multiplier = -1.0;
    }

    // Score on left collision
    if ball_transform.translation.x < LEFT + BALL_RADIUS {
        score.player_two += 1;
        right_scoreboard_text.sections[0].value = score.player_two.to_string();
        x_multiplier = -1.0;
    }

    // Reflect ball according to multipliers
    ball.velocity.x *= x_multiplier;
    ball.velocity.y *= y_multiplier;

    // Update ball position
    ball_transform.translation += ball.velocity;
}
