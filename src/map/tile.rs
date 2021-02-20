use crate::resource::{Resource};
use crate::utility::{Coord};
use crate::buyable::{town::Town};


#[derive(Debug)]
pub struct Tile {
    resource: Resource,
    towns: Vec<Box<dyn Town>>,
    coord: Coord
}
