mod basic_heightmap;
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

    basic_heightmap::execute(&mut world, &tileset);
    set_world_wall::execute(&mut world, &tileset);
    set_grass_layer::execute(&mut world, &tileset);
    set_stone_areas::execute(&mut world, &tileset);

    set_spawn_point::execute(&mut world);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}
