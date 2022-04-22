use bevy::{log, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{
    components::{Death, GameDirection, Jumper},
    constants::CAMERA_SPEED_PER_SEC,
    events::BulletFiredEvent,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_jumps.system())
                    .with_system(player_controller.system())
                    .with_system(player_camera_control.system())
                    .with_system(fire_controller.system())
                    .with_system(jump_reset.system()),
            )
            .add_system(print_size);

        // #[cfg(feature = "debug")]
        // {
        //     app.register_inspectable::<CollisionEvent>();
        // }

        log::info!("Physics Build Complete!");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0., 2.).into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        forces: RigidBodyForces {
            gravity_scale: 3.,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::round_cuboid(56., 72., 0.1).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    //let img = asset.get(asset_server.load("../assets/sprites/Death.png"));

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            texture: asset_server.load("../assets/sprites/Death.png"),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Jumper {
            jump_impulse: 1000.0,
            is_jumping: false,
        })
        .insert(Death {
            speed: 30.0,
            facing_direction: GameDirection::Right,
        });
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

pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut send_fire_event: EventWriter<BulletFiredEvent>,
    players: Query<(&Death, &RigidBodyPositionComponent), With<Death>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (player, position) in players.iter() {
            let event = BulletFiredEvent {
                position: Vec2::new(
                    position.position.translation.x,
                    position.position.translation.y,
                ),
                direction: player.facing_direction,
            };
            send_fire_event.send(event);
        }
    }
}

pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Death, &mut RigidBodyVelocityComponent)>,
) {
    for (mut player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
            player.facing_direction = GameDirection::Left
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
            player.facing_direction = GameDirection::Right
        }
    }
}

fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut RigidBodyVelocityComponent), With<Death>>,
) {
    for (jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
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

fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &ContactEvent) {
    if let ContactEvent::Started(h1, h2) = event {
        if h1.entity() == entity || h2.entity() == entity {
            jumper.is_jumping = false
        }
    }
}

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
