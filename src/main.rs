use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Hello, world!",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                font: asset_server.load("FiraSans-Bold.ttf")
            },
            Default::default()
        ),
        ..Default::default()
    });
}