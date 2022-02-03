use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 800.0;
const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 3.0;

#[derive(Component)]
struct Paddle {
    up_key: KeyCode,
    down_key: KeyCode,
}

#[derive(Component)]
struct Ball;


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
        .add_system(paddle_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new((-SCREEN_WIDTH / 2.0) + (PADDLE_WIDTH / 2.0), 0.0, 1.0),
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle {
            up_key: KeyCode::W,
            down_key: KeyCode::S,
        });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new((SCREEN_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0), 0.0, 1.0),
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle {
            up_key: KeyCode::Up,
            down_key: KeyCode::Down,
        });
}

fn paddle_movement(mut paddle_query: Query<(&mut Transform, &Paddle)>, keyboard_input: Res<Input<KeyCode>>) {
    paddle_query.iter_mut().for_each(|(mut transform, paddle)| {
        let y = &mut transform.translation.y;
        if keyboard_input.pressed(paddle.up_key) {
            *y += PADDLE_SPEED;
        }
        
        if keyboard_input.pressed(paddle.down_key) {
            *y += -PADDLE_SPEED;
        }
        *y = y.min((SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0).max((-SCREEN_HEIGHT + PADDLE_HEIGHT) / 2.0);
    });
}