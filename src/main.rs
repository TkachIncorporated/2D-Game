use bevy::{core::FixedTimestep, prelude::*};

//Frames Imitation
const TIME_STEP: f32 = 1.0 / 60.0;

//States System for Pause & Play (TODO)
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MainMenu,
    Paused,
    Playing,
    GameOver,
}
//Position Component for Grid Map (TODO)
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

//Trying to make Square Collider (TODO)
#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
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
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new().with_system(size_scaling),
            )
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_velocity)
                .with_system(movement),
        );
    }
}

fn main() {
    App::new()
        .add_plugin(SetupPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(WeaponPlugin)
        .run();
}

//Doing things with Death
fn size_scaling(mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width * 3 as f32,
            sprite_size.height * 3 as f32,
            1.0,
        );
    }
}

//Doing spawning things with Death
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../Sprites/Death.png"),
            ..default()
        })
        .insert(Death)
        .insert(Size::square(1.0));
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
        position.translation.x -= 2.;
        renderer.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        position.translation.x += 2.;
        renderer.flip_x = false;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        position.translation.y -= 2.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        position.translation.y += 2.;
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
