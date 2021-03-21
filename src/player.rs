use crate::resource::{ResourceDeck};
use crate::buyable::{Buyable, development_card::DevelopmentCard};
use crate::map::{tile::Tile};
use crate::utility::{Deck};


#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub cards: Deck,
    pub buildings: Vec<i32>
}

impl Player {
    pub fn add_resources(&mut self, resources: ResourceDeck){
        self.cards.resource_cards += resources;
    }
    pub fn sub_resources(&mut self, resources: ResourceDeck){
        self.cards.resource_cards -= resources;
    }
    pub fn gain_resources(&mut self, tiles: Vec<Tile>){
        for tile in &tiles {
            for town in tile.towns.iter() {
                if town.get_player() == 1 { // todo: == self
                    self.add_resources(
                        ResourceDeck::new_vec(vec![(tile.resource, town.resource_multiplier())]));
                }
            }
        }
    }
    pub fn buy(&self, to_buy: i32){} // maybe redo struct
    pub fn can_buy(&self, cost: ResourceDeck){}
    pub fn play_card(&self, card: DevelopmentCard){} // useless?
    pub fn trade(&self, gain:  ResourceDeck, traded:  ResourceDeck){}
    pub fn calculate_points(&self) -> i32{
        1
    }
}
