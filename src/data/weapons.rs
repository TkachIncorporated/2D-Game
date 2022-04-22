use bevy::{core::FixedTimestep, log, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::data::constants::TIME_STEP;

use super::{
    components::{GameDirection, RangedWeapon},
    events::BulletFiredEvent,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        )
        .add_system(weapon)
        .add_system(destroy_bullet_on_contact);

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
    mut bullet_fired_events: EventReader<BulletFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        spawn_bullet(&mut commands, asset_server.clone(), event)
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: AssetServer,
    options: &BulletFiredEvent,
) {
    let speed = match options.direction {
        GameDirection::Left => -140.0,
        _ => 140.0,
    };

    let x = match options.direction {
        GameDirection::Left => options.position.x - 70.,
        _ => options.position.x + 70.,
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
        shape: ColliderShape::cuboid(0.25, 0.05).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let sprite = SpriteBundle {
        texture: asset_server.load("../assets/sprites/WTFIsThis.png"),
        sprite: Sprite {
            custom_size: Vec2::new(50., 20.).into(),
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(RangedWeapon::scythe(100., false));

    log::info!("Bullet Spawned!");
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<RangedWeapon>>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for bullet in bullets.iter() {
                if h1.entity() == bullet || h2.entity() == bullet {
                    commands.entity(bullet).despawn_recursive();

                    log::info!("Bullet Destroyed!");
                }
            }
        }
    }
}
