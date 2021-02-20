pub mod tile;
use tile::Tile;
use crate::buyable::{town::Town, road::Road};


#[derive(Debug)]
pub struct Map {
    thief: i32,
    tiles: Vec<Tile>, // meant to be 2d array
    towns: Vec<Box<dyn Town>>, // meant to be 2d array
    roads: Vec<Road>, // meant to be 2d array
}


impl Map {
    fn move_thief(&mut self, new_position: i32) -> bool {
        let res = self.thief != new_position;
        if self.thief != new_position {
            self.thief = new_position;
        }
        res
    }
}
