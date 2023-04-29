use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldStorage {
    tiles: Vec<u32>,
    width: usize,
    height: usize,
    spawn_point: usize,
}

#[allow(dead_code)]
impl WorldStorage {
    pub fn from_dimensions(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![0; width * height],
            width,
            height,
            spawn_point: 0,
        }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_spawn_point(&self) -> UVec2 {
        self.delinearize(self.spawn_point)
    }

    pub fn set_spawn_point(&mut self, x: u32, y: u32) {
        self.spawn_point = self.linearize(x as usize, y as usize);
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    #[inline]
    pub fn linearize(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    #[inline]
    pub fn delinearize(&self, idx: usize) -> UVec2 {
        let x = idx % self.width;
        let y = idx / self.width;
        UVec2::new(x as u32, y as u32)
    }

    #[inline]
    pub fn get_tile(&self, x: i32, y: i32) -> u32 {
        assert!(x >= 0 && y >= 0);

        self.get_tile_idx(self.linearize(x as usize, y as usize))
    }

    #[inline]
    pub fn get_tile_idx(&self, idx: usize) -> u32 {
        self.tiles[idx]
    }

    #[inline]
    pub fn set_tile(&mut self, x: i32, y: i32, tile: u32) {
        assert!(x >= 0 && y >= 0);

        self.set_tile_idx(self.linearize(x as usize, y as usize), tile);
    }

    #[inline]
    pub fn set_tile_idx(&mut self, idx: usize, tile: u32) {
        self.tiles[idx] = tile;
    }
}
