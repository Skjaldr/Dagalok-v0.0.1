use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_third_person_camera::{Offset, ThirdPersonCamera, Zoom};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_light));
    }
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    },
    ThirdPersonCamera {
        offset_enabled: true,
        zoom: Zoom::new(1.0, 3.0),
        offset: Offset::new(0.0, 0.12),
        ..default()
    }
    ));


}

pub fn spawn_light(
    mut commands: Commands
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.50, 0.0),
            rotation: Quat::from_rotation_x(-PI / 6.),
            ..default()
        },

        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 5.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}