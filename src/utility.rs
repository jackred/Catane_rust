use rand::Rng;

use crate::resource::ResourceDeck;
use crate::buyable::development_card::DevelopmentCard;

#[derive(Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}

impl Coord {
    pub fn is_connected(&self, coord: &Coord) -> bool {
        match (self.y<6, self.y%2) {
            (_,_) if coord.x == self.x && coord.y == self.y+1 => true,
            (_,_) if coord.x == self.x && coord.y == self.y-1 => true,
            (true, 0) if coord.x == self.x+1 && coord.y == self.y+1 => true,
            (true, 1) if coord.x == self.x-1 && coord.y == self.y-1 => true,
            (false, 0) if coord.x == self.x-1 && coord.y == self.y+1 => true,
            (false, 1) if coord.x == self.x+1 && coord.y == self.y-1 => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    pub resource_cards: ResourceDeck,
    pub research_cards:  Vec<DevelopmentCard>
}


fn roll_dices(value: i32) -> (i32, i32) {
    (rand::thread_rng().gen_range(1, value),
     rand::thread_rng().gen_range(1, value))
}

pub fn roll_6_dices() -> (i32, i32){
    roll_dices(6)
}
