mod basic_heightmap;
mod dig_caves;
mod dig_tunnels;
mod set_grass_layer;
mod set_spawn_point;
mod set_stone_areas;
mod set_world_wall;

use bevy::prelude::*;
use bevy_tileset::prelude::Tilesets;

use crate::states::GameState;

use super::storage::WorldStorage;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate.in_schedule(OnEnter(GameState::WorldGeneration)));
    }
}

fn generate(mut commands: Commands, tilesets: Tilesets) {
    let tileset = tilesets.get_by_name("world_tiles").unwrap();
    let mut world = WorldStorage::from_dimensions(4200, 1200);

    let mut rng = rand::thread_rng();

    // Set initial terrain tiles
    basic_heightmap::execute(&mut rng, &mut world, &tileset);
    set_grass_layer::execute(&mut world, &tileset);
    set_stone_areas::execute(&mut rng, &mut world, &tileset);

    // Dig features into terrain
    dig_caves::execute(&mut rng, &mut world);
    dig_tunnels::execute(&mut rng, &mut world);

    // Set necessary world features
    set_world_wall::execute(&mut world, &tileset);
    set_spawn_point::execute(&mut world);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}
