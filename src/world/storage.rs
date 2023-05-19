use bevy::prelude::*;

use super::world_pos::WorldTilePos;

#[derive(Resource)]
pub struct WorldStorage {
    tiles: Vec<u32>,
    walls: Vec<u32>,
    width: u32,
    height: u32,
    spawn_point_idx: usize,
}

#[allow(dead_code)]
impl WorldStorage {
    pub fn from_dimensions(width: u32, height: u32) -> Self {
        Self {
            tiles: vec![0; (width * height) as usize],
            walls: vec![0; (width * height) as usize],
            width,
            height,
            spawn_point_idx: 0,
        }
    }

    #[inline]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_spawn_point(&self) -> WorldTilePos {
        self.delinearize(self.spawn_point_idx)
    }

    pub fn set_spawn_point(&mut self, tile_pos: WorldTilePos) {
        self.spawn_point_idx = self.linearize(tile_pos);
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    #[inline]
    pub fn linearize(&self, tile_pos: WorldTilePos) -> usize {
        (tile_pos.x() + self.width * tile_pos.y()) as usize
    }

    #[inline]
    pub fn delinearize(&self, idx: usize) -> WorldTilePos {
        let x = idx as u32 % self.width;
        let y = idx as u32 / self.width;
        WorldTilePos::new(x, y)
    }

    #[inline]
    pub fn get_tile(&self, pos: WorldTilePos) -> u32 {
        self.get_tile_idx(self.linearize(pos))
    }

    #[inline]
    pub fn get_tile_idx(&self, idx: usize) -> u32 {
        self.tiles[idx]
    }

    #[inline]
    pub fn set_tile(&mut self, pos: WorldTilePos, tile: u32) {
        self.set_tile_idx(self.linearize(pos), tile);
    }

    #[inline]
    pub fn set_tile_idx(&mut self, idx: usize, tile: u32) {
        self.tiles[idx] = tile;
    }

    #[inline]
    pub fn get_wall(&self, pos: WorldTilePos) -> u32 {
        self.get_wall_idx(self.linearize(pos))
    }

    #[inline]
    pub fn get_wall_idx(&self, idx: usize) -> u32 {
        self.walls[idx]
    }

    #[inline]
    pub fn set_wall(&mut self, pos: WorldTilePos, tile: u32) {
        self.set_wall_idx(self.linearize(pos), tile);
    }

    #[inline]
    pub fn set_wall_idx(&mut self, idx: usize, wall: u32) {
        self.walls[idx] = wall;
    }
}
