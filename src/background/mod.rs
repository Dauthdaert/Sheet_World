use bevy::prelude::*;
use bevy_parallax::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.4, 0.95),
                    path: "backgrounds/sky.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 0.0,
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.2, 0.05),
                    path: "backgrounds/clouds_1.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 1.0,
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.05, 0.05),
                    path: "backgrounds/clouds_2.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 2.0,
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.05, 0.05),
                    path: "backgrounds/rocks_1.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 3.0,
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.05, 0.05),
                    path: "backgrounds/clouds_3.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 4.0,
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.05, 0.05),
                    path: "backgrounds/rocks_2.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 5.0,
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.05, 0.05),
                    path: "backgrounds/clouds_4.png".to_string(),
                    tile_size: Vec2::new(1920.0, 1080.0),
                    cols: 1,
                    rows: 1,
                    scale: 2.0,
                    z: 6.0,
                    ..default()
                },
            ],
            ..default()
        });

        app.add_plugin(ParallaxPlugin);
    }
}
