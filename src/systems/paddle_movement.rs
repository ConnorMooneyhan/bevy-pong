use crate::prelude::*;

pub fn paddle_movement_system(mut paddle_query: Query<(&mut Transform, &Paddle)>, keyboard_input: Res<Input<KeyCode>>) {
    paddle_query.iter_mut().for_each(|(mut transform, paddle)| {
        let y = &mut transform.translation.y;
        if keyboard_input.pressed(paddle.up_key) {
            *y += PADDLE_SPEED;
        }
        
        if keyboard_input.pressed(paddle.down_key) {
            *y += -PADDLE_SPEED;
        }
        *y = y.min(TOP - PADDLE_HEIGHT / 2.0).max(BOTTOM + PADDLE_HEIGHT / 2.0);
    });

}
