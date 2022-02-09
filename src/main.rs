#![warn(clippy::all, clippy::pedantic)]

mod systems;
mod components;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::sprite::collide_aabb::{ collide, Collision };
    pub const SCREEN_WIDTH: f32 = 1200.0;
    pub const SCREEN_HEIGHT: f32 = 800.0;
    pub const PADDLE_WIDTH: f32 = 12.0;
    pub const PADDLE_HEIGHT: f32 = 100.0;
    pub const PADDLE_SPEED: f32 = 4.0;
    pub const BALL_RADIUS: f32 = 5.0;
    pub const BALL_SPEED: f32 = 5.0;
    pub const TOP: f32 = SCREEN_HEIGHT / 2.0;
    pub const BOTTOM: f32 = -SCREEN_HEIGHT / 2.0;
    pub const RIGHT: f32 = SCREEN_WIDTH / 2.0;
    pub const LEFT: f32 = -SCREEN_WIDTH / 2.0;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::GameState;
}

use prelude::*;

pub struct Scoreboard {
    pub player_one: i32,
    pub player_two: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
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
        .insert_resource(GameState::Playing)
        .insert_resource(Scoreboard {
            player_one: 0,
            player_two: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(playing_systems())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn left Paddle
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

    // Spawn right Paddle
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

    // Spawn ball
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
