use bevy_tileset::prelude::Tileset;

use crate::world::{WorldStorage, WorldTilePos};

pub fn execute(world: &mut WorldStorage, tileset: &Tileset) {
    let tile = tileset.get_tile_index("WorldBorder").unwrap();

    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            if y == 0 || x == 0 || y == world.get_height() - 1 || x == world.get_width() - 1 {
                world.set_tile(WorldTilePos::new(x, y), *tile.base_index() as u32);
            }
        }
    }
}
