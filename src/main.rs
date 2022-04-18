use bevy::prelude::*;

//States System for Pause & Play (TODO)
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}
//Position Component for Grid Map (TODO)
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

//Trying to make Square Collider
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

//Main Hero
#[derive(Component)]
struct Death;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(size_scaling),
        )
        .add_system(movement)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../Sprites/Death.png"),
            sprite: Sprite {
                flip_x: true,
                flip_y: false,
                ..default()
            },
            ..default()
        })
        .insert(Death)
        .insert(Size::square(1.0));
}

//Doing aggressive things with Death
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
