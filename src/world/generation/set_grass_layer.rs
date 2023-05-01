use bevy_tileset::prelude::Tileset;

use crate::world::WorldStorage;

pub fn execute(world: &mut WorldStorage, tileset: &Tileset) {
    let grass = tileset.get_tile_index("Grass").unwrap();
    let dirt = tileset.get_tile_index("Dirt").unwrap();

    for x in 0..world.get_width() as i32 {
        for y in (0..world.get_height() as i32).rev() {
            if world.get_tile(x, y) == *dirt.base_index() as u32 {
                world.set_tile(x, y, *grass.base_index() as u32);
                break;
            }
        }
    }
}
