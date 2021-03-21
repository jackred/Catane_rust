use crate::resource::{Resource, ResourceDeck};
use crate::buyable::Buyable;
use crate::utility::{Coord};

#[derive(Debug)]
pub struct City {
    location: Coord,
    player: i32, // player to be implemented
    harbor: Option<bool> // harbor type to be implemented
}

impl City {
    pub fn new(location: Coord, player: i32, harbor: Option<bool>) -> City{
        City {
            location: location,
            player: player,
            harbor: harbor
        }
    }
}

impl super::Town for City {
    #[inline]
    fn point(&self) -> i32 {
        2
    }
    #[inline]
    fn resource_multiplier(&self) -> i32 {
        2
    }
    
    fn get_player(&self) -> i32 {
        self.player
    }
}

impl Buyable for City  {
    #[inline]
    fn get_cost() ->ResourceDeck {
        ResourceDeck(enum_map! {
            Resource::Ore => 3,
            Resource::Grain => 2,
            _ => 0,
        })
    }
}
