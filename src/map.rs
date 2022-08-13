pub mod tile;
use tile::Tile;
use crate::buyable::{town::Town, road::Road};
use crate::utility::{Coord};


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

    fn create_tiles_arrays() -> Vec<Vec<Tile>>{
	let mut map = vec![];
	let schema = [3, 4, 5, 4, 3];
	for n in 0..5 {
	    map.push(vec![]);
	    for m in 0..schema[n] {
		if n == 2 && m == 2 {
		    map[n].push(Tile{hex: tile::TileType::Desert, towns: vec![], coord: Coord{y: n as i32, x:m}, number:0})
		}
		//map[n].push(Tile);
	    }
	}
	
	
	return map;
    }
}
