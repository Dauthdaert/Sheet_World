use bevy_tileset::prelude::Tileset;
use noise::{NoiseFn, Perlin};

use crate::world::WorldStorage;

pub fn execute(world: &mut WorldStorage, tileset: &Tileset) {
    let heightmap = Perlin::new(0);
    let base_terrain_height = 700;
    let terrain_height: Vec<i32> = (0..world.get_width())
        .map(|x| base_terrain_height + (heightmap.get([x as f64 / 200.0]) * 40.0) as i32)
        .collect();

    let tile_index = tileset.get_tile_index("Dirt").unwrap();
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let mut tile = 0;
            if y <= terrain_height[x] as usize {
                tile = *tile_index.base_index() as u32;
            }

            let idx = world.linearize(x, y);
            world.set_tile_idx(idx, tile);
        }
    }
}
