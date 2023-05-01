use bevy_tileset::prelude::Tileset;
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, RngCore};

use crate::world::{WorldStorage, MIN_TILE_ID};

pub fn execute(rng: &mut ThreadRng, world: &mut WorldStorage, tileset: &Tileset) {
    let stone_map = Perlin::new(rng.next_u32());

    let tile_index = tileset.get_tile_index("Stone").unwrap();
    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let x_noise = x as f64 / 50.0;
            let y_noise = y as f64 / 50.0;
            let stone_val = (stone_map.get([x_noise, y_noise]) + 1.0) * 50000.0;
            let stone_limit = (y / 6 * y) as f64;

            let idx = world.linearize(x, y);
            if stone_val > stone_limit && world.get_tile_idx(idx) > MIN_TILE_ID {
                world.set_tile_idx(idx, *tile_index.base_index() as u32);
            }
        }
    }
}
