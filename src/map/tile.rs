use crate::resource::{Resource};
use crate::utility::{Coord};
use crate::buyable::{town::Town};


#[derive(Debug)]
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

fn assign_tiles() {

}
