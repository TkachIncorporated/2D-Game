use bevy::{
    core::FixedTimestep,
    input::Input,
    log,
    math::Vec3,
    prelude::{
        default, App, AssetServer, Commands, KeyCode, Plugin, Query, Res, SystemSet, Transform,
        With,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    components::{death::Death, rangedweapon::RangedWeapon},
    events::CollisionEvent,
    TIME_STEP,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(apply_weapon_velocity),
            )
            .add_system(weapon);

        // #[cfg(feature = "debug")]
        // {
        //     app.register_inspectable::<CollisionEvent>();
        // }

        log::info!("Weapon Build Complete!");
    }
}

//TODO Doing deadly things by Death
fn weapon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut transform: Query<&Transform, With<Death>>,
    mut sprite: Query<&mut Sprite, With<Death>>,
) {
    let position = transform.single_mut();
    let renderer = sprite.single_mut();
    let mut need_spawn = false;

    if keyboard_input.just_pressed(KeyCode::Z) {
        need_spawn = true;
    }

    if need_spawn {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("../Sprites/WTFIsThis.png"),
                transform: Transform {
                    translation: Vec3::new(
                        position.translation.x + (if renderer.flip_x { -1.0 } else { 1.0 }) * 100.0, //Targeting thingies
                        position.translation.y,
                        position.translation.z,
                    ),
                    scale: position.scale,
                    ..default()
                },
                sprite: Sprite {
                    flip_x: renderer.flip_x,
                    ..default()
                },
                ..default()
            })
            .insert(RangedWeapon::scythe(100.0, renderer.flip_x));
    }
}

//TODO Weapon Physics
pub fn apply_weapon_velocity(mut query: Query<(&mut Transform, &RangedWeapon)>) {
    for (mut transform, head) in query.iter_mut() {
        transform.translation.x += head.velocity * (if head.flip { -1.0 } else { 1.0 });
    }
}
