use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, RngCore};

use crate::world::WorldStorage;

pub fn execute(rng: &mut ThreadRng, world: &mut WorldStorage) {
    let cave_map = Perlin::new(rng.next_u32());

    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let x_noise = x as f64 / 80.0;
            let y_noise = y as f64 / 80.0;
            let cave_val = cave_map.get([x_noise, y_noise]);
            let cave_limit = 0.6;

            let idx = world.linearize(x, y);
            if cave_val > cave_limit {
                world.set_tile_idx(idx, 0);
            }
        }
    }
}
