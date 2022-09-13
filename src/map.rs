pub mod tile;
use tile::{Tile, TileType};
use crate::buyable::{town::Town, road::Road};
use crate::utility::{Coord};

// https://stackoverflow.com/a/23810557/8040287
pub static TOKENS: &'static [i32] = &[10, 2, 9, 12, 6, 4, 10, 9, 11, 3, 8, 8, 3, 4, 5, 5, 6, 11];
pub static SCHEMA: &'static [i32] = &[3, 4, 5, 4, 3];



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

    pub fn create_tiles_arrays(layout: Vec<TileType>, arg_tokens: Option<Vec<i32>>, arg_schema: Option<Vec<i32>>) -> Vec<Vec<Tile>> {
	let mut map = vec![];
	let schema = arg_schema.unwrap_or(SCHEMA.to_vec());
	let tokens = arg_tokens.unwrap_or(TOKENS.to_vec());
	let mut i = 0;
	let mut j = 0;

	for n in 0..5 {
	    map.push(vec![]);
	    for m in 0..schema[n] {
		let mut number = tokens[j];
		if layout[i] == TileType::Desert {
		    number = 7;
		} else {
		    j += 1;
		}
		map[n].push(Tile{hex: layout[i], towns: vec![], coord: Coord{y: n as i32, x:m}, number: number});   

		i += 1;
	    }
	}
	
	
	return map;
    }
}
