mod assets_paths;
mod components;
mod constants;
mod events;
mod grid;
mod menu;
mod player;
mod weapons;

use bevy::{app::AppExit, log, prelude::*};
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_rapier2d::prelude::*;

use crate::AppState;

use self::{player::PlayerPlugin, weapons::WeaponPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainTest)
                .with_system(setup)
                .with_system(start_background_audio),
        )
        .add_plugin(AudioPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WeaponPlugin);
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<self::menu::MenuMaterials>()
            .add_system(self::menu::button_system.system())
            .add_system(button_press_system_main_menu.system())
            .add_system(button_press_system_level_menu.system())
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(cleanup.system())
                    .with_system(setup_main_menu.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup.system()))
            .add_system_set(
                SystemSet::on_enter(AppState::LevelSelectMenu)
                    .with_system(cleanup.system())
                    .with_system(setup_select_menu.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::LevelSelectMenu).with_system(cleanup.system()),
            )
            .add_system_set(SystemSet::on_enter(AppState::MainTest).with_system(cleanup.system()))
            .add_system_set(SystemSet::on_exit(AppState::MainTest).with_system(cleanup.system()))
            .add_system_set(SystemSet::on_enter(AppState::LightTest).with_system(cleanup.system()))
            .add_system_set(SystemSet::on_exit(AppState::LightTest).with_system(cleanup.system()));
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<self::menu::MenuMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(self::menu::root(&materials))
        .with_children(|parent| {
            parent
                .spawn_bundle(self::menu::border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(self::menu::menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(self::menu::button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(self::menu::button_text(
                                        &asset_server,
                                        &materials,
                                        "New Game",
                                    ));
                                })
                                .insert(MenuButton::Play);
                            if !cfg!(target_arch = "wasm32") {
                                parent
                                    .spawn_bundle(self::menu::button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn_bundle(self::menu::button_text(
                                            &asset_server,
                                            &materials,
                                            "Quit",
                                        ));
                                    })
                                    .insert(MenuButton::Quit);
                            }
                        });
                });
        });
}

fn button_press_system_main_menu(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => {
                    log::info!("Load level select menu.");

                    state
                        .set(AppState::LevelSelectMenu)
                        .expect("Couldn't switch state to SelectLevelMenu")
                }

                MenuButton::Quit => {
                    log::info!("Quit from game.");

                    exit.send(AppExit)
                }
            };
        }
    }
}

#[derive(Component)]
enum SelectButton {
    Main,
    Light,
}

fn setup_select_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<self::menu::MenuMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(self::menu::root(&materials))
        .with_children(|parent| {
            parent
                .spawn_bundle(self::menu::border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(self::menu::menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(self::menu::button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(self::menu::button_text(
                                        &asset_server,
                                        &materials,
                                        "Main",
                                    ));
                                })
                                .insert(SelectButton::Main);
                            if !cfg!(target_arch = "wasm32") {
                                parent
                                    .spawn_bundle(self::menu::button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn_bundle(self::menu::button_text(
                                            &asset_server,
                                            &materials,
                                            "Light",
                                        ));
                                    })
                                    .insert(SelectButton::Light);
                            }
                        });
                });
        });
}

fn button_press_system_level_menu(
    buttons: Query<(&Interaction, &SelectButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                SelectButton::Main => {
                    log::info!("Load main test.");

                    state
                        .set(AppState::MainTest)
                        .expect("Couldn't switch state to MainTest")
                }

                SelectButton::Light => {
                    log::info!("Load light text.");

                    state
                        .set(AppState::LightTest)
                        .expect("Couldn't switch state to LightTest")
                }
            };
        }
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load(assets_paths::sounds::WANDERING);

    audio.play_looped(music);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(assets_paths::sprites::TODO),
            transform: Transform {
                translation: Vec3::new(0., -250., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0., -250.).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::round_cuboid(220. / 2., 22. / 2., 0.1).into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(250., 0., 0.),
                scale: Vec3::new(30., 500., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(250., 0.).into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::round_cuboid(30. / 2., 500. / 2., 0.1).into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
}
