use bevy_tileset::prelude::Tileset;

use crate::world::{WorldStorage, MIN_TILE_ID};

pub fn execute(world: &mut WorldStorage, tileset: &Tileset) {
    let tile = tileset.get_tile_index("Grass").unwrap();
    for x in 0..world.get_width() as i32 {
        for y in (0..world.get_height() as i32).rev() {
            if world.get_tile(x, y) > MIN_TILE_ID {
                world.set_tile(x, y, *tile.base_index() as u32);
                break;
            }
        }
    }
}
