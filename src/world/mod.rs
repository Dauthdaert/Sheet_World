mod chunks;
pub use chunks::LoadPoint;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::TilemapRenderSettings;
use bevy_tileset::prelude::Tileset;

use crate::states::GameState;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct TileTextures {
    #[asset(path = "tileset.ron")]
    tileset: Handle<Tileset>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: chunks::RENDER_CHUNK_SIZE,
            ..default()
        })
        .add_plugin(bevy_ecs_tilemap::TilemapPlugin);
        app.add_plugin(bevy_tileset::prelude::TilesetPlugin::default());

        app.add_collection_to_loading_state::<_, TileTextures>(GameState::AssetLoading);

        app.init_resource::<chunks::LoadedChunks>();
        app.add_systems(
            (
                chunks::despawn_chunks_far_from_camera,
                chunks::spawn_chunks_around_camera,
            )
                .in_set(OnUpdate(GameState::InGame)),
        );
    }
}