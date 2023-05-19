use crate::world::{world_pos::WorldTilePos, WorldStorage, MIN_TILE_ID};

pub fn execute(world: &mut WorldStorage) {
    let middle_x = world.get_width() / 2;
    for y in (0..world.get_height()).rev() {
        if world.get_tile(WorldTilePos::new(middle_x, y)) >= MIN_TILE_ID {
            world.set_spawn_point(WorldTilePos::new(middle_x, y - 1));
            break;
        }
    }
}
