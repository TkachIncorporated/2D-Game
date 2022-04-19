use bevy::prelude::Component;

//TODO RangedWeapon Component
#[derive(Debug, Copy, Clone, Component)]
pub struct RangedWeapon {
    pub velocity: f32,
    pub flip: bool,
}

impl RangedWeapon {
    pub fn scythe(velocity: f32, flip: bool) -> Self {
        Self {
            velocity: velocity,
            flip: flip,
        }
    }
}
