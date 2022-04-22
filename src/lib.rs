mod data;

use bevy::{prelude::*, window::WindowMode};
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use data::GamePlugin;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    BetweenLevels,
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(WindowDescriptor {
        title: "Platformer!".to_string(),
        width: 640.0,
        height: 400.0,
        mode: WindowMode::Windowed,
        resizable: false,
        ..Default::default()
    })
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(DefaultPlugins)
    .add_state(AppState::InGame)
    .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    .add_plugin(GamePlugin)
    .run();
}
