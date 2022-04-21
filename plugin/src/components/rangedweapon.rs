use bevy::prelude::Component;

//TODO RangedWeapon Component
#[derive(Debug, Copy, Clone, Component)]
pub struct RangedWeapon {
    pub range: f32,
    pub velocity: f32,
    pub flip: bool,
}

impl RangedWeapon {
    pub fn scythe(range: f32, velocity: f32, flip: bool) -> Self {
        Self {
            range: range,
            velocity: velocity,
            flip: flip,
        }
    }
}
