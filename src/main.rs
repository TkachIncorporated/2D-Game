use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use heron::prelude::*;
use plugin::MainPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugin(SetupPlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81 * 3.0, 0.0)))
        .add_plugin(MainPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
