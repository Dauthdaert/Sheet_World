use bevy::{prelude::*, render::view::RenderLayers};
use bevy_parallax::ParallaxCameraComponent;

use crate::world::LoadPoint;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerCamera;

pub fn spawn(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.7;

    commands.spawn((
        camera_bundle,
        RenderLayers::layer(0),
        UiCameraConfig { show_ui: false },
        LoadPoint::new(3),
        ParallaxCameraComponent,
        PlayerCamera,
    ));
}
