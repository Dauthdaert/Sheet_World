use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct WorldTilePos {
    x: u32,
    y: u32,
}

impl WorldTilePos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_world_pos(world_pos: Vec2, tile_size: Vec2) -> Self {
        let converted_pos = (world_pos / tile_size).floor().as_uvec2();
        Self {
            x: converted_pos.x,
            y: converted_pos.y,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
