use crate::resource::{Resource};
use crate::utility::{Coord};
use crate::buyable::{town::Town};


#[derive(Debug)]
pub struct Tile {
    pub resource: Resource,
    pub towns: Vec<Box<dyn Town>>,
    pub coord: Coord
}
