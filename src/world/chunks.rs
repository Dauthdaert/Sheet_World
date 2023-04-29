use bevy::{math::Vec3Swizzles, prelude::*, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;

use crate::player::PlayerCamera;

use super::storage::WorldStorage;

const CHUNK_SIZE: UVec2 = UVec2 { x: 64, y: 64 };
const I_CHUNK_SIZE: IVec2 = IVec2 {
    x: CHUNK_SIZE.x as i32,
    y: CHUNK_SIZE.y as i32,
};
pub(super) const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Component, Clone, Copy, Debug)]
pub struct LoadPoint {
    radius: u32,
}

impl LoadPoint {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct RenderedChunks {
    loaded: HashMap<IVec2, Entity>,
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    tilesets: Tilesets,
    world_storage: Res<WorldStorage>,
    camera_query: Query<(&Transform, &LoadPoint), With<PlayerCamera>>,
    mut rendered_chunks: ResMut<RenderedChunks>,
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();

    for (transform, load_point) in camera_query.iter() {
        let camera_chunk_pos =
            camera_pos_to_chunk_pos(transform.translation.xy(), tileset.tile_size());

        for y in (camera_chunk_pos.y - load_point.radius as i32)
            ..(camera_chunk_pos.y + load_point.radius as i32)
        {
            for x in (camera_chunk_pos.x - load_point.radius as i32)
                ..(camera_chunk_pos.x + load_point.radius as i32)
            {
                let chunk_pos = IVec2::new(x, y);
                if !rendered_chunks.loaded.contains_key(&chunk_pos) {
                    let chunk = spawn_chunk(&mut commands, tileset, &world_storage, chunk_pos);
                    rendered_chunks.loaded.insert(chunk_pos, chunk);
                }
            }
        }
    }
}

pub fn despawn_chunks_far_from_camera(
    mut commands: Commands,
    tilesets: Tilesets,
    camera_query: Query<(&Transform, &LoadPoint), With<PlayerCamera>>,
    chunks_query: Query<(Entity, &Transform), With<TileStorage>>,
    mut rendered_chunks: ResMut<RenderedChunks>,
) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();

    for (camera_transform, load_point) in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            if camera_transform
                .translation
                .xy()
                .distance(chunk_transform.translation.xy())
                > (load_point.radius * CHUNK_SIZE.x) as f32 * tileset.tile_size().x * 2.0
            {
                let chunk_pos =
                    camera_pos_to_chunk_pos(chunk_transform.translation.xy(), tileset.tile_size());
                rendered_chunks.loaded.remove(&chunk_pos);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn camera_pos_to_chunk_pos(camera_pos: Vec2, tile_size: Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let tile_size = tile_size.as_ivec2();
    camera_pos / (I_CHUNK_SIZE * tile_size)
}

fn spawn_chunk(
    commands: &mut Commands,
    tileset: &Tileset,
    world: &WorldStorage,
    chunk_pos: IVec2,
) -> Entity {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let tile_size = tileset.tile_size();
    let chunk_transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * tile_size.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * tile_size.y,
        10.0,
    ));

    let tileset_handle = tileset.texture();

    commands
        .entity(tilemap_entity)
        .with_children(|builder| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let tile_pos = TilePos { x, y };

                    let tile_pos_x = chunk_pos.x * CHUNK_SIZE.x as i32 + tile_pos.x as i32;
                    let tile_pos_y = chunk_pos.y * CHUNK_SIZE.y as i32 + tile_pos.y as i32;

                    let tile_index = if !world.in_bounds(tile_pos_x, tile_pos_y) {
                        0
                    } else {
                        world.get_tile(tile_pos_x, tile_pos_y)
                    };

                    let tile_entity = builder
                        .spawn(TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(tile_index),
                            tilemap_id: TilemapId(builder.parent_entity()),
                            ..default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(TilemapBundle {
            grid_size: tile_size.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(tileset_handle.clone()),
            tile_size: tile_size.into(),
            transform: chunk_transform,
            ..default()
        })
        .id()
}
