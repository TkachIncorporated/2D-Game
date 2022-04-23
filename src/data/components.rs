use bevy::prelude::Component;

//TODO Death Component
#[derive(Copy, Clone, Component)]
pub struct Death {
    pub speed: f32,
    pub facing_direction: GameDirection,
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

//TODO Ground Component
#[derive(Debug, Copy, Clone, Component)]
pub struct Ground;
//TODO RangedWeapon Component
#[derive(Debug, Copy, Clone, Component)]
pub struct RangedWeapon {
    pub range: f32,
}

impl RangedWeapon {
    pub fn scythe(range: f32) -> Self {
        Self { range }
    }
}

#[derive(Copy, Clone)]
pub enum GameDirection {
    Left,
    Right,
}
