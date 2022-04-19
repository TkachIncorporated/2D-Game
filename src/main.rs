use bevy::{
    core::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

//Frames Imitation
const TIME_STEP: f32 = 1.0 / 60.0;

//States System for Pause & Play (TODO)
#[derive(Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    Paused,
    Playing,
    GameOver,
}

//Real Collider This Time
#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

//Ground Flag for Furute prikols
#[derive(Component)]
struct Ground;

//Rigid-Body Component
const G: f32 = -9.8;

#[derive(Component)]
struct RigidBody {
    moveable: bool,
    velocity: Vec2,
}

//Main Hero Component
#[derive(Component)]
struct Death;

//Weapon Component
#[derive(Component)]
struct RangedWeapon {
    velocity: f32,
    flip: bool,
}
impl RangedWeapon {
    fn scythe(velocity: f32, flip: bool) -> Self {
        Self {
            velocity: velocity,
            flip: flip,
        }
    }
}

//Some Plugins for eyes comfort
pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(weapon); //Will increase when the time comes
    }
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_startup_system(setup)
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

pub struct CharactersPlugin;
impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(apply_velocity)
                .with_system(apply_physics.before(check_for_collisions))
                .with_system(movement),
        );
    }
}

fn main() {
    let mut app = App::new();

    ////TODO
    // app.add_plugin(SetupPlugin)
    //     .add_plugin(LogDiagnosticsPlugin::default())
    //     .add_plugin(FrameTimeDiagnosticsPlugin::default())
    //     .add_plugin(CharactersPlugin)
    //     .add_plugin(WeaponPlugin)
    //     .add_plugins(DefaultPlugins);

    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

//Doing spawning things with Death
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

//Doing aggressive things with Death (TODO)
fn movement(
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

//Doing deadly things by Death (TODO)
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

fn apply_velocity(mut query: Query<(&mut Transform, &RangedWeapon)>) {
    for (mut transform, head) in query.iter_mut() {
        transform.translation.x += head.velocity * (if head.flip { -1.0 } else { 1.0 });
    }
}

//Physics Immitation (TODO)
fn apply_physics(
    mut query: Query<(&mut Transform, &RigidBody)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for (mut transform, head) in query.iter_mut() {
        if head.moveable {
            if !collision_events.iter().next().is_some() {
                transform.translation.y += head.velocity.y;
            }
        }
    }
}

//Collision Check (TODO)
fn check_for_collisions(
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
