use bevy::{
    core::FixedTimestep,
    log,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    components::{collider::Collider, death::Death, rigidbody::RigidBody},
    events::CollisionEvent,
    G, TIME_STEP,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(check_for_collisions)
                    .with_system(apply_physics.before(check_for_collisions))
                    .with_system(movement),
            )
            .add_startup_system(setup);

        // #[cfg(feature = "debug")]
        // {
        //     app.register_inspectable::<CollisionEvent>();
        // }

        log::info!("Physics Build Complete!");
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
