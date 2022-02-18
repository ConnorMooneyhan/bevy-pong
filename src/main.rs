#![warn(clippy::all, clippy::pedantic)]

mod components;
mod systems;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::sprite::collide_aabb::{collide, Collision};
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
    pub const SCOREBOARD_SIZE: f32 = 48.0;
    pub const GOAL_SCORE: i32 = 5;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::GameState;
    pub use crate::Score;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Pong".to_string(),
            ..Default::default()
        })
        .insert_resource(Score {
            player_one: 0,
            player_two: 0,
        })
        .add_state(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(setup_cameras)
            .with_system(set_paddles)
            .with_system(set_ball)
            .with_system(set_scoreboards))
        .add_system_set(playing_systems())
        .add_system(systems::end_game_system)
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(clear_screen))
        .run();
}

pub struct Score {
    pub player_one: i32,
    pub player_two: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn set_paddles(mut commands: Commands) {
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
}

fn set_ball(mut commands: Commands) {
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
            velocity: Vec3::new(BALL_SPEED, BALL_SPEED, 0.0),
        });
}

fn set_scoreboards(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
    // Scoreboard helper variables
    let text_style = TextStyle {
        font: asset_server.load("FiraSans-Bold.ttf"),
        font_size: SCOREBOARD_SIZE,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    // Spawn left scoreboard
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                score.player_one.to_string(),
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform {
                translation: Vec3::new(
                    LEFT + (SCOREBOARD_SIZE / 2.0) + 5.0,
                    TOP - (SCOREBOARD_SIZE / 2.0) - 5.0,
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LeftScoreboard);

    // Spawn right scoreboard
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                score.player_two.to_string(),
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform {
                translation: Vec3::new(
                    RIGHT - (SCOREBOARD_SIZE / 2.0) - 5.0,
                    TOP - (SCOREBOARD_SIZE / 2.0) - 5.0,
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RightScoreboard);
}

fn clear_screen(mut commands: Commands, entities: Query<Entity>) {
    entities.for_each(|entity| commands.entity(entity).despawn_recursive());
}
