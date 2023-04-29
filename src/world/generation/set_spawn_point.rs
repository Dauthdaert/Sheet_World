use crate::world::WorldStorage;

pub fn execute(world: &mut WorldStorage) {
    let middle_x = world.get_width() / 2;
    for y in (0..world.get_height()).rev() {
        if world.get_tile(middle_x as i32, y as i32) > 1 {
            world.set_spawn_point(middle_x as u32, y as u32 - 1);
            break;
        }
    }
}
