use bevy_tileset::prelude::Tileset;
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, RngCore};

use crate::world::{world_pos::WorldTilePos, WorldStorage};

pub fn execute(
    rng: &mut ThreadRng,
    world: &mut WorldStorage,
    tileset: &Tileset,
    wallset: &Tileset,
) {
    let heightmap = Perlin::new(rng.next_u32());
    let base_terrain_height = 700;
    let terrain_height: Vec<u32> = (0..world.get_width())
        .map(|x| (base_terrain_height + (heightmap.get([x as f64 / 200.0]) * 40.0) as i32) as u32)
        .collect();

    let tile_index = tileset.get_tile_index("Dirt").unwrap();
    let wall_index = wallset.get_tile_index("DirtNatural").unwrap();
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let mut tile = 0;
            let mut wall = 0;
            if y <= terrain_height[x as usize] {
                tile = *tile_index.base_index() as u32;
                wall = *wall_index.base_index() as u32;
            }

            let idx = world.linearize(WorldTilePos::new(x, y));
            world.set_tile_idx(idx, tile);
            world.set_wall_idx(idx, wall);
        }
    }
}
