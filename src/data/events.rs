use bevy::math::Vec2;

use super::components::GameDirection;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Default)]
pub struct SomeEvent;

pub struct BulletFiredEvent {
    pub position: Vec2,
    pub direction: GameDirection,
}
