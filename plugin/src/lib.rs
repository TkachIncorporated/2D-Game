//FrameRate Immitation
const TIME_STEP: f32 = 1.0 / 60.0;
//Camera Speed
const CAMERA_SPEED_PER_SEC: f32 = 5.0;
//Player Speed
const SPEED: f32 = 200.0;

mod components;
mod events;
mod player;
mod weapons;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use player::PlayerPlugin;
use weapons::WeaponPlugin;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin).add_plugin(WeaponPlugin);
    }
}
