use bevy::prelude::*;
use bevy::sprite::collide_aabb::{ collide, Collision };

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 800.0;
const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 4.0;
const BALL_RADIUS: f32 = 5.0;
const BALL_SPEED: f32 = 5.0;

const TOP: f32 = SCREEN_HEIGHT / 2.0;
const BOTTOM: f32 = -SCREEN_HEIGHT / 2.0;
const RIGHT: f32 = SCREEN_WIDTH / 2.0;
const LEFT: f32 = -SCREEN_WIDTH / 2.0;

#[derive(Component)]
struct Paddle {
    up_key: KeyCode,
    down_key: KeyCode,
}

#[derive(Component)]
struct Ball {
    velocity: Vec3
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Pong".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(paddle_movement_system)
        .add_system(ball_collision_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Left Paddle
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(LEFT + (PADDLE_WIDTH / 2.0), 0.0, 1.0),
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle {
            up_key: KeyCode::W,
            down_key: KeyCode::S,
        });

    // Right Paddle
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(RIGHT - (PADDLE_WIDTH / 2.0), 0.0, 1.0),
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle {
            up_key: KeyCode::Up,
            down_key: KeyCode::Down,
        });

    // Ball
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: Vec3::new(BALL_SPEED, BALL_SPEED, 0.0)
        });
}

fn paddle_movement_system(mut paddle_query: Query<(&mut Transform, &Paddle)>, keyboard_input: Res<Input<KeyCode>>) {
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

fn ball_collision_system(paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>, mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    let (mut ball, mut ball_transform) = ball_query.single_mut();
    let mut x_multiplier = 1.0;
    let mut y_multiplier = 1.0;

    for paddle_transform in paddle_query.iter() {
        // Collide with paddle
        let collision = collide(
            ball_transform.translation,
            ball_transform.scale.truncate(),
            paddle_transform.translation,
            paddle_transform.scale.truncate()
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