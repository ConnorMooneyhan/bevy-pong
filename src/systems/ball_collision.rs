use crate::prelude::*;

pub fn ball_collision_system(
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
) {
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

    // Reflect off edges of screen
    if ball_transform.translation.y > TOP - BALL_RADIUS {
        y_multiplier = -1.0;
    }
    if ball_transform.translation.y < BOTTOM + BALL_RADIUS {
        y_multiplier = -1.0;
    }
    if ball_transform.translation.x > RIGHT - BALL_RADIUS {
        x_multiplier = -1.0;
    }
    if ball_transform.translation.x < LEFT + BALL_RADIUS {
        x_multiplier = -1.0;
    }

    // Reflect ball according to multipliers
    ball.velocity.x *= x_multiplier;
    ball.velocity.y *= y_multiplier;

    // Update ball position
    ball_transform.translation += ball.velocity;
}
