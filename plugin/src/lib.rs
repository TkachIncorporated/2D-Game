//Gravity Immitation
const G: f32 = -9.8;
//FrameRate Immitation
const TIME_STEP: f32 = 1.0 / 60.0;

mod components;
mod events;
mod physics;
mod weapons;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use physics::PhysicsPlugin;
use weapons::WeaponPlugin;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin).add_plugin(WeaponPlugin);
    }
}
