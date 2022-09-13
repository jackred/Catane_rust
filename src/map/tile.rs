use enum_map::EnumMap;
use crate::resource::{Resource};
use crate::utility::{Coord};
use crate::buyable::{town::Town};

#[derive(Debug, Enum, Copy, Clone, PartialEq)]
pub enum TileType {
    Field,
    Forest,
    Mountain,
    Pasture,
    Hill,
    Desert
}

#[derive(Debug)]
pub struct Tile {
    pub hex: TileType,
    pub towns: Vec<Box<dyn Town>>,
    pub coord: Coord,
    pub number: i32
}

impl Tile {

    pub fn get_resource(&self) -> Option<Resource> {
	match self.hex {
	    TileType::Field => Some(Resource::Grain),
	    TileType::Forest => Some(Resource::Lumber),
	    TileType::Mountain => Some(Resource::Ore),
	    TileType::Pasture => Some(Resource::Wool),
	    TileType::Hill => Some(Resource::Brick),
	    _ => None
	}
    }

    pub fn has_resource(&self) -> bool {
	match self.hex {
	    TileType::Field |
	    TileType::Forest |
	    TileType::Mountain |
	    TileType::Pasture |
	    TileType::Hill => true,
	    _ => false
	}
    }
    
}

fn get_tiletype_by_number() -> EnumMap<TileType, i32> {
    enum_map!{
	TileType::Desert => 1,
	TileType::Forest | TileType::Field | TileType::Pasture => 4,
	TileType::Mountain | TileType::Hill => 3
    }
}


pub fn get_standard_layout() -> Vec<TileType> {
    vec![
	TileType::Mountain, TileType::Pasture, TileType::Forest,
	TileType::Field, TileType::Hill, TileType::Pasture, TileType::Hill,
	TileType::Field, TileType::Forest, TileType::Desert, TileType::Forest, TileType::Mountain,
	TileType::Forest, TileType::Mountain, TileType::Field, TileType::Pasture,
	TileType::Hill, TileType::Field, TileType::Pasture]
}

// use an enum to generate an array then does permutation
fn get_random_layout() -> Vec<TileType>{
    vec![]
}

