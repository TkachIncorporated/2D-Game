use bevy::{math::Vec2, prelude::Component};

//TODO RigidBody Component
#[derive(Debug, Copy, Clone, Component)]
pub struct RigidBody {
    pub moveable: bool,
    pub velocity: Vec2,
}
