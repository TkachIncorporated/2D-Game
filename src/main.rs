mod data;

use bevy::{prelude::*, window::WindowMode};
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    BetweenLevels,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Death".to_string(),
        width: 640.0,
        height: 400.0,
        mode: WindowMode::Windowed,
        ..Default::default()
    })
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(DefaultPlugins)
    .add_state(AppState::MainMenu)
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugin(data::MenuPlugin)
    .add_plugin(data::GamePlugin)
    .run();
}
