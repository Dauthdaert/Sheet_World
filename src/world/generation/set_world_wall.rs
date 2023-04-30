use bevy_tileset::prelude::Tileset;

use crate::world::WorldStorage;

pub fn execute(world: &mut WorldStorage, tileset: &Tileset) {
    let tile = tileset.get_tile_index("WorldBorder").unwrap();

    for y in 0..world.get_height() {
        for x in 0..world.get_width() {
            if y == 0 || x == 0 || y == world.get_height() - 1 || x == world.get_width() - 1 {
                world.set_tile(x as i32, y as i32, *tile.base_index() as u32);
            }
        }
    }
}
