use bevy_tileset::prelude::Tileset;

use crate::world::{WorldStorage, WorldTilePos};

pub fn execute(world: &mut WorldStorage, tileset: &Tileset) {
    let grass = tileset.get_tile_index("Grass").unwrap();
    let dirt = tileset.get_tile_index("Dirt").unwrap();

    for x in 0..world.get_width() {
        for y in (0..world.get_height()).rev() {
            let idx = world.linearize(WorldTilePos::new(x, y));
            if world.get_tile_idx(idx) == *dirt.base_index() as u32 {
                world.set_tile_idx(idx, *grass.base_index() as u32);
                break;
            }
        }
    }
}
