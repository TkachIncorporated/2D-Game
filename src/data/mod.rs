pub mod components;
pub mod constants;
pub mod events;

pub mod player;
pub mod weapons;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use self::{player::PlayerPlugin, weapons::WeaponPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
            .add_plugin(PlayerPlugin)
            .add_plugin(WeaponPlugin)
            .add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../assets/sprites/TODO.png"),
            transform: Transform {
                translation: Vec3::new(0., -250., 0.),
                scale: Vec3::new(3., 3., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0., -250.).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::round_cuboid(300., 75., 0.1).into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(250., 0., 0.),
                scale: Vec3::new(30., 500., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(250., 0.).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::round_cuboid(30., 500., 0.1).into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
}
