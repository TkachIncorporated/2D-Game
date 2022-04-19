//Gravity Immitation
const G: f32 = -9.8;
//FrameRate Immitation
const TIME_STEP: f32 = 1.0 / 60.0;

mod components;
mod events;

use crate::events::*;
use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::{log, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use components::collider::Collider;
use components::death::Death;
use components::rangedweapon::RangedWeapon;
use components::rigidbody::RigidBody;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(check_for_collisions)
                    .with_system(apply_weapon_velocity)
                    .with_system(apply_physics.before(check_for_collisions))
                    .with_system(movement),
            )
            .add_system(weapon)
            .add_startup_system(setup);

        // #[cfg(feature = "debug")]
        // {
        //     app.register_inspectable::<CollisionEvent>();
        // }

        log::info!("Gravity Build Complete!");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(50.0, 50.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            //texture: asset_server.load("../Sprites/Death.png"),
            ..default()
        })
        .insert(Death)
        .insert(RigidBody {
            moveable: true,
            velocity: Vec2::new(0.0, G),
        });

    commands
        .spawn_bundle(SpriteBundle {
            //texture: asset_server.load("../Sprites/TODO.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -250.0, 0.0),
                scale: (Vec3::new(10000.0, 30.0, 1.0)),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody {
            moveable: false,
            velocity: Vec2::new(0.0, 0.0),
        })
        .insert(Collider);
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

//TODO Doing aggressive things with Death
pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform: Query<&mut Transform, With<Death>>,
    mut sprite: Query<&mut Sprite, With<Death>>,
) {
    let mut position = transform.single_mut();
    let mut renderer = sprite.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        position.translation.x -= 5.0;
        renderer.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        position.translation.x += 5.0;
        renderer.flip_x = false;
    }
}

//TODO Physics Immitation
pub fn apply_physics(
    mut query: Query<(&mut Transform, &RigidBody)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for (mut transform, head) in query.iter_mut() {
        if head.moveable {
            if let None = collision_events.iter().next() {
                transform.translation.y += head.velocity.y;
            }
        }
    }
}

//TODO Collision Check
pub fn check_for_collisions(
    mut death_query: Query<(&mut RigidBody, &Transform), With<Death>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut rb, death_transform) = death_query.single_mut();
    let death_size = death_transform.scale.truncate();

    //Check collision with walls
    for transform in collider_query.iter() {
        let collision = collide(
            death_transform.translation,
            death_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            let mut y_movement = true;
            let mut x_movement = true;

            match collision {
                Collision::Left => x_movement = false,
                Collision::Right => x_movement = false,
                Collision::Top => y_movement = false,
                Collision::Bottom => y_movement = false,
                Collision::Inside => { /* do nothing */ }
            }

            if !y_movement {
                rb.velocity.y = 0.0;
            }

            if !x_movement {
                rb.velocity.x = 0.0;
            }
        } else {
            rb.velocity.y = G;
        }
    }
}

//TODO Weapon Physics
pub fn apply_weapon_velocity(mut query: Query<(&mut Transform, &RangedWeapon)>) {
    for (mut transform, head) in query.iter_mut() {
        transform.translation.x += head.velocity * (if head.flip { -1.0 } else { 1.0 });
    }
}
