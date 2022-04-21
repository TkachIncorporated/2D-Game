use crate::{
    components::{death::Death, layers::Layer, rangedweapon::RangedWeapon},
    TIME_STEP,
};
use bevy::{core::FixedTimestep, log, prelude::*};
use heron::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_weapon_physics),
        )
        .add_system(weapon)
        .add_system(destroy_bullet_on_contact)
        .add_system(log_collisions);

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
                texture: asset_server.load("../assets/sprites/WTFIsThis.png"),
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
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(20.0, 10.0, 1.0),
                border_radius: Some(0.0),
            })
            .insert(RangedWeapon::scythe(100.0, 20.0, renderer.flip_x))
            .insert(CollisionLayers::new(Layer::Bullet, Layer::Enemy));
    }
}

//TODO Weapon Physics
pub fn apply_weapon_physics(mut query: Query<(&mut Transform, &RangedWeapon)>) {
    for (mut transform, head) in query.iter_mut() {
        transform.translation.x += head.velocity * (if head.flip { -1.0 } else { 1.0 });
    }
}

pub fn destroy_bullet_on_contact(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_bullet(layers_1) && is_enemy(layers_2) {
                Some(entity_2)
            } else if is_bullet(layers_2) && is_enemy(layers_1) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|enemy_entity| commands.entity(enemy_entity).despawn());
}

fn is_bullet(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Bullet) && !layers.contains_group(Layer::Enemy)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Bullet) && layers.contains_group(Layer::Enemy)
}

fn log_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {
                println!("Collision started between {:?} and {:?}", d1, d2)
            }
            CollisionEvent::Stopped(d1, d2) => {
                println!("Collision stopped between {:?} and {:?}", d1, d2)
            }
        }
    }
}
