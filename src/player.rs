use crate::resource::{ResourceDeck};
use crate::buyable::{Buyable, development_card::DevelopmentCard};
use crate::map::{tile::Tile};
use crate::utility::{Deck};


#[derive(Debug)]
pub struct Player {
    name: String,
    card: Deck,
    buildings: Vec<i32>
}

impl Player {
    pub fn add_resources(&self, resources: ResourceDeck){}
    pub fn rm_resources(&self, resources: ResourceDeck){}
    pub fn gain_resources(&self, tiles: Vec<Tile>){}
    pub fn buy(&self, buyable: Box<dyn Buyable>){} // maybe redo struct
    pub fn can_buy(&self, cost: ResourceDeck){}
    pub fn play_card(&self, card: DevelopmentCard){} // useless?
    pub fn trade(&self, gain:  ResourceDeck, traded:  ResourceDeck){}
    pub fn calculate_points(&self) -> i32{
        1
    }
}
