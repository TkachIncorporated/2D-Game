use bevy::{log, prelude::*};
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

use super::{components, events};
use crate::{
    data::{assets_paths, constants},
    AppState,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::MainTest)
                .with_system(weapon.system())
                .with_system(destroy_bullet_on_contact.system()),
        );

        // #[cfg(feature = "debug")]
        // {
        //     app.register_inspectable::<CollisionEvent>();
        // }

        log::info!("Weapon Build Complete!");
    }
}

fn weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bullet_fired_events: EventReader<events::BulletFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        spawn_bullet(&mut commands, asset_server.clone(), event)
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: AssetServer,
    options: &events::BulletFiredEvent,
) {
    let speed = match options.direction {
        components::GameDirection::Left => -constants::BULLET_SPEED,
        _ => constants::BULLET_SPEED,
    };

    let x = match options.direction {
        components::GameDirection::Left => options.position.x - 70.,
        _ => options.position.x + 70.,
    };

    let flip = match options.direction {
        components::GameDirection::Left => true,
        _ => false,
    };

    let rigid_body = RigidBodyBundle {
        position: Vec2::new(x, options.position.y).into(),
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(speed, 0.0).into(),
            ..Default::default()
        }
        .into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        forces: RigidBodyForces {
            gravity_scale: 0.,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(23. / 2., 13. / 2.).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let sprite = SpriteBundle {
        texture: asset_server.load(assets_paths::sprites::WTF_IS_THIS),
        sprite: Sprite {
            flip_x: flip,
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(components::RangedWeapon::scythe(100.));

    log::info!("Bullet Spawned!");
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<components::RangedWeapon>>,
    mut contact_events: EventReader<ContactEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for bullet in bullets.iter() {
                if h1.entity() == bullet || h2.entity() == bullet {
                    let sound = asset_server.load(assets_paths::sounds::FIREBALL_WALL);
                    commands.entity(bullet).despawn_recursive();

                    audio.set_volume(0.5);
                    audio.play(sound);

                    log::info!("Bullet Destroyed!");
                }
            }
        }
    }
}
