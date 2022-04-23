use bevy::math::Vec2;

use super::components;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct BulletFiredEvent {
    pub position: Vec2,
    pub direction: components::GameDirection,
}
