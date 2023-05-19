use noise::{Fbm, NoiseFn, Perlin};
use rand::{rngs::ThreadRng, RngCore};

use crate::world::{world_pos::WorldTilePos, WorldStorage};

pub fn execute(rng: &mut ThreadRng, world: &mut WorldStorage) {
    let tunnel_map = Fbm::<Perlin>::new(rng.next_u32());

    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            let x_noise = x as f64 / 80.0;
            let y_noise = y as f64 / 80.0;
            let tunnel_val = tunnel_map.get([x_noise, y_noise]);
            let tunnel_limit = 0.13;

            if tunnel_val > 0.0 && tunnel_val < tunnel_limit {
                let idx = world.linearize(WorldTilePos::new(x, y));
                world.set_tile_idx(idx, 0);
            }
        }
    }
}
