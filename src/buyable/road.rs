use crate::resource::{Resource, ResourceDeck};
use crate::buyable::Buyable;
use crate::utility::{Coord};

type CoordRoad =  (Coord, Coord);

#[derive(Debug)]
pub struct Road { 
    location: CoordRoad,
    connected: Vec<Road>,
    player: i32
}

impl Road {
    pub fn new(north_coord: Coord, south_coord: Coord, player: i32) -> Road {
        Road {
            location: (north_coord, south_coord),
            connected: vec!(),
            player: player
        }
    }
    // not sure of that
    // I feel like I did, are the 2 spots connected
    // rather than equal
    // which may totally be a mistake xD
    pub fn is_connected(&self, road: &Road) -> bool {
        self.location.0.is_connected(&road.location.0)
            ||  self.location.0.is_connected(&road.location.1)
            ||  self.location.1.is_connected(&road.location.0)
            ||  self.location.1.is_connected(&road.location.1)
    }
}

impl Buyable for Road  {
    #[inline]
    fn get_cost() -> ResourceDeck {
        ResourceDeck(enum_map! {
            Resource::Lumber => 1,
            Resource::Brick => 1,
            _ => 0,
        })
    }
}

