use bevy::{prelude::*, render::view::RenderLayers};
use bevy_parallax::ParallaxCameraComponent;
use bevy_tileset::prelude::Tilesets;

use crate::world::{LoadPoint, WorldStorage};

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerCamera;

pub fn spawn(mut commands: Commands, tilesets: Tilesets, world: Res<WorldStorage>) {
    let mut camera_bundle = Camera2dBundle::default();

    // Scale camera to show appropriate number of tiles on screen
    camera_bundle.projection.scale = 0.7;

    // Move camera to spawn point
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let tile_size = tileset.tile_size();
    let spawn_point = world.get_spawn_point();
    camera_bundle.transform.translation.x = spawn_point.x as f32 * tile_size.x;
    camera_bundle.transform.translation.y = spawn_point.y as f32 * tile_size.y;

    commands.spawn((
        camera_bundle,
        RenderLayers::layer(0),
        UiCameraConfig { show_ui: false },
        LoadPoint::new(3),
        ParallaxCameraComponent,
        PlayerCamera,
    ));
}
