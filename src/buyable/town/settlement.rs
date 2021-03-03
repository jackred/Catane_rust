use crate::resource::{Resource, ResourceDeck};
use crate::buyable::Buyable;
use crate::utility::{Coord};
use super::city::City;

#[derive(Debug)]
pub struct Settlement {
    location: Coord,
    player: i32, // player to be implemented
    harbor: Option<bool> // harbor type to be implemented
}

impl Settlement {
    pub fn new(location: Coord, player: i32, harbor: Option<bool>) -> Settlement {
        Settlement {
            location: location,
            player: player,
            harbor: harbor
        }
    }
    
    pub fn get_player(&self) -> i32 {
        self.player
    }

    pub fn evolve_town(self) -> City {
        City::new(self.location, self.player, self.harbor)
    }
}

impl super::Town for Settlement {
    #[inline]
    fn point(&self) -> i32 {
        1
    }

    #[inline]
    fn resource_multiplier(&self) -> i32 {
        1
    }
}

impl Buyable for Settlement  {
    #[inline]
    fn get_cost() -> ResourceDeck {
        enum_map! {
            Resource::Lumber => 1,
            Resource::Brick => 1,
            Resource::Wool => 1,
            Resource::Grain => 1,
            _ => 0,
        }
    }
}
