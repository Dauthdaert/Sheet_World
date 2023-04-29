use bevy::prelude::*;
use bevy_parallax::ParallaxCameraComponent;

use crate::world::LoadPoint;

pub fn spawn(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.7;

    commands.spawn((camera_bundle, LoadPoint::new(3), ParallaxCameraComponent));
}
