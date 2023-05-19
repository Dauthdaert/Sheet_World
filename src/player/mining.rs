use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileStorage;
use bevy_tileset::prelude::Tilesets;

use crate::world::{RenderedChunks, WorldStorage, WorldTilePos, MIN_TILE_ID, MIN_WALL_ID};

use super::PlayerCamera;

#[allow(clippy::too_many_arguments)]
pub fn mine(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    tilesets: Tilesets,
    mut world_storage: ResMut<WorldStorage>,
    mut commands: Commands,
    loaded_chunks: Res<RenderedChunks>,
    mut chunk_query: Query<&mut TileStorage>,
) {
    let window = windows.single();
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let (camera, camera_global_transform) = camera_query.single();

    // FIXME: Currently has trouble finding the correct tile to select based on cursor position.
    if mouse_input.just_pressed(MouseButton::Left) {
        let Some(world_cursor_pos) = camera.viewport_to_world_2d(camera_global_transform, cursor_pos) else { return; };
        if world_cursor_pos.x < 0.0 || world_cursor_pos.y < 0.0 {
            return;
        }

        let tileset = tilesets.get_by_name("world_tiles").unwrap();
        let tile_size = tileset.tile_size();

        let tile_cursor_pos = WorldTilePos::from_world_pos(world_cursor_pos, tile_size);
        let tile_cursor_idx = world_storage.linearize(tile_cursor_pos);

        if world_storage.get_tile_idx(tile_cursor_idx) >= MIN_TILE_ID {
            world_storage.set_tile_idx(tile_cursor_idx, 0);
            let (tile_chunk, tile_pos) = loaded_chunks.get_tile_chunk(tile_cursor_pos);
            let mut tile_storage = chunk_query.get_mut(tile_chunk).unwrap();

            let tile = tile_storage.get(&tile_pos).unwrap();
            commands.entity(tile).despawn();
            tile_storage.remove(&tile_pos);
        } else if world_storage.get_wall_idx(tile_cursor_idx) >= MIN_WALL_ID {
            world_storage.set_wall_idx(tile_cursor_idx, 0);
            let (tile_chunk, tile_pos) = loaded_chunks.get_wall_chunk(tile_cursor_pos);
            let mut tile_storage = chunk_query.get_mut(tile_chunk).unwrap();

            let tile = tile_storage.get(&tile_pos).unwrap();
            commands.entity(tile).despawn();
            tile_storage.remove(&tile_pos);
        }
    }
}
