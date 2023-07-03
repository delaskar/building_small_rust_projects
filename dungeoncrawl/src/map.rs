use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

