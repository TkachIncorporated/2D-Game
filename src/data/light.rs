use bevy::{
    log,
    math::{Quat, Vec2, Vec3},
    pbr::{
        AlphaMode, AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, PointLight,
        PointLightBundle, StandardMaterial,
    },
    prelude::{
        shape, AssetServer, Assets, BuildChildren, Color, Commands, Handle, Mesh,
        OrthographicCameraBundle, OrthographicProjection, PerspectiveCameraBundle, Plugin, Res,
        ResMut, SystemSet, Transform,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{data::assets_paths, AppState};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(Self::init));
    }
}

impl LightPlugin {
    pub fn init(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        // some quad

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load(assets_paths::sprites::DEATH)),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            // transform: Transform::from_xyz(0.0, 0.5, 0.0),
            // materials.add(asset_server.load(assets_paths::sprites::DEATH).into()),
            ..Default::default()
        });
        // cube
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            texture: asset_server.load(assets_paths::sprites::DEATH),
            ..Default::default()
        });
        // light
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        });
        // camera
        let mut camera = OrthographicCameraBundle::new_3d();
        camera.orthographic_projection.scale = 3.0;
        camera.transform = Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Y);
        commands.spawn_bundle(camera);

        log::info!("llll");
    }
}
