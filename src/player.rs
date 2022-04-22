mod components;

use bevy::{core::FixedTimestep, log, prelude::*};
use heron::{prelude::*, rapier_plugin::rapier2d::prelude::ContactEvent};

use crate::{
    components::{death::Death, jumper::Jumper, layers::Layer},
    CAMERA_SPEED_PER_SEC, SPEED, TIME_STEP,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(movement)
                .with_system(player_camera_control),
        )
        .add_startup_system(setup)
        .add_system(player_jumps)
        .add_system(print_size);

        // #[cfg(feature = "debug")]
        // {
        //     app.register_inspectable::<CollisionEvent>();
        // }

        log::info!("Physics Build Complete!");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let img = asset.get(asset_server.load("../assets/sprites/Death.png"));
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            texture: asset_server.load("../assets/sprites/Death.png"),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(56.0 / 2.0, 72.0 / 2.0, 0.0), //TODO IDK how to get scales of image
            border_radius: Some(0.0),
        })
        .insert(Velocity::default())
        .insert(RotationConstraints::lock())
        .insert(Jumper {
            jump_impulse: 1000.0,
            is_jumping: false,
        })
        .insert(Death);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../assets/sprites/TODO.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -250.0, 0.0),
                scale: Vec3::new(3.0, 3.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(300.0 / 2.0, 75.0 / 2.0, 1.0),
            border_radius: Some(0.0),
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(250.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 500.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(30.0 / 2.0, 500.0 / 2.0, 1.0),
            border_radius: None,
        })
        .insert(CollisionLayers::new(Layer::Enemy, Layer::Bullet));
}

fn print_size(
    query: Query<(&Sprite, &Transform, Option<&Handle<Image>>)>,
    images: Res<Assets<Image>>,
) {
    info!("sizes:");

    for (sprite, t, opt_handle) in query.iter() {
        let size = if let Some(custom_size) = sprite.custom_size {
            custom_size
        } else if let Some(image) = opt_handle.map(|handle| images.get(handle)).flatten() {
            Vec2::new(
                image.texture_descriptor.size.width as f32,
                image.texture_descriptor.size.height as f32,
            )
        } else {
            Vec2::new(1.0, 1.0)
        };

        info!("{:?}", size * t.scale.truncate());
    }
}

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite: Query<(&mut Sprite, &mut Velocity), With<Death>>,
) {
    for (mut sprite, mut velocity) in sprite.iter_mut() {
        let x = if keyboard_input.pressed(KeyCode::Left) {
            sprite.flip_x = true;
            -20.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            sprite.flip_x = false;
            20.0
        } else {
            0.0
        };

        let target_velocity = Vec2::new(x, -9.8).normalize_or_zero().extend(0.0) * SPEED;

        velocity.linear = target_velocity;
    }
}

fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Death>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linear = Vec3::new(0.0, jumper.jump_impulse, 1.0);
            jumper.is_jumping = true
        }
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

// fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &ContactEvent) {
//     if let ContactEvent::Started(h1, h2) = event {
//         if h1.entity() == entity || h2.entity() == entity {
//             jumper.is_jumping = false
//         }
//     }
// }

fn player_camera_control(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<Death>>,
) {
    let dist = CAMERA_SPEED_PER_SEC * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if kb.pressed(KeyCode::PageUp) {
            log_scale -= dist;
        }
        if kb.pressed(KeyCode::PageDown) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}

//TODO Collision Check (Might be useful for bullet collision checking)
// pub fn check_for_collisions(
//     mut death_query: Query<(&mut RigidBody, &Transform), With<Death>>,
//     collider_query: Query<&Transform, With<Collider>>,
//     mut collision_events: EventWriter<CollisionEvent>,
// ) {
//     let (mut rb, death_transform) = death_query.single_mut();
//     let death_size = death_transform.scale.truncate();

//     //Check collision with walls
//     for transform in collider_query.iter() {
//         let collision = collide(
//             death_transform.translation,
//             death_size,
//             transform.translation,
//             transform.scale.truncate(),
//         );

//         if let Some(collision) = collision {
//             collision_events.send_default();

//             let mut y_movement = true;
//             let mut x_movement = true;

//             match collision {
//                 Collision::Left => x_movement = false,
//                 Collision::Right => x_movement = false,
//                 Collision::Top => y_movement = false,
//                 Collision::Bottom => y_movement = false,
//                 Collision::Inside => { /* do nothing */ }
//             }

//             if !y_movement {
//                 rb.velocity.y = 0.0;
//             }

//             if !x_movement {
//                 rb.velocity.x = 0.0;
//             }
//         } else {
//             rb.velocity.y = G;
//         }
//     }
// }
