#![allow(dead_code)]
use std::io;
use rand::Rng;
// use std::collections::HashMap;
use core::fmt::Debug;

#[macro_use]
extern crate enum_map;


mod resource;
mod buyable;
use buyable::town::Town;


#[derive(Debug)]
enum TypeBuyable {
    DevelopmentCard,
    Settlement,
    City,
    Road
}


#[derive(Debug)]
pub struct Coord {
    x: i32,
    y: i32
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



mod map {
    #[derive(Debug)]
    pub struct Map {
        thief: i32,
        tiles: Vec<super::tile::Tile>, // meant to be 2d array
        towns: Vec<Box<dyn super::buyable::town::Town>>, // meant to be 2d array
        roads: Vec<super::buyable::road::Road>, // meant to be 2d array
    }


    impl Map {
        fn move_thief(&mut self, new_position: i32) -> bool {
            let res = self.thief != new_position;
            if self.thief != new_position {
                self.thief = new_position;
            }
            res
        }
    }
}

mod tile {
    #[derive(Debug)]
    pub struct Tile {
        resource: super::resource::Resource,
        towns: Vec<Box<dyn super::buyable::town::Town>>,
        coord: super::Coord
    }
}



mod player {
    #[derive(Debug)]
    pub struct Player {
        name: String,
        card: super::Deck,
        buildings: Vec<i32>
    }

    impl Player {
        pub fn add_resources(&self, resources: super::resource::ResourceDeck){}
        pub fn rm_resources(&self, resources: super::resource::ResourceDeck){}
        pub fn gain_resources(&self, tiles: Vec<super::tile::Tile>){}
        pub fn buy(&self, buyable: Box<dyn super::buyable::Buyable>){} // maybe redo struct
        pub fn can_buy(&self, cost: super::resource::ResourceDeck){}
        pub fn play_card(&self, card: super::buyable::development_card::DevelopmentCard){} // useless?
        pub fn trade(&self, gain:  super::resource::ResourceDeck, traded:  super::resource::ResourceDeck){}
        pub fn calculate_points(&self) -> i32{
            1
        }
    }
}


#[derive(Debug)]
struct Deck {
    resource_cards: resource::ResourceDeck,
    research_cards:  Vec<buyable::development_card::DevelopmentCard>
}


fn roll_dices(value: i32) -> (i32, i32) {
    (rand::thread_rng().gen_range(1, value),
     rand::thread_rng().gen_range(1, value))
}

fn roll_6_dices() -> (i32, i32){
    roll_dices(6)
}




fn main() {
    println!("Please, enter your name: ");
    let mut name = String::new(); 
    io::stdin().read_line(&mut name)
        .expect("Failed to read the line");
    println!("Your name is : {}", name);
    println!("Rolling the dices...");
    let (d1, d2) = roll_6_dices();
    println!("Dice 1: {}, Dice 2: {}", d1, d2);
    let map = enum_map! {
        resource::Resource::Grain => 32,
        _ => 0,
    };
    println!("{:?}", map[resource::Resource::Ore]);
    let deck = Deck {
        resource_cards: map,
        research_cards: buyable::development_card::DevelopmentCard::create_deck()
    };
    println!("{:?}", deck);
    //println!("Town price: {:?}", get_cost(TypeBuyable::Town));
    deck.research_cards[0].effect();
    let r = buyable::road::Road::new(Coord{x:1,y:1}, Coord{x:2,y:2});
    println!("{:?}", r);
    let r2 = buyable::road::Road::new(Coord{x:1,y:0}, Coord{x:8,y:-8});
    println!("{:?}", r2);
    println!("{:?}", r.is_connected(&r2));
    println!("{:?}", r2.is_connected(&r));
    let s = buyable::town::settlement::Settlement::new(Coord{x: 5, y: 7}, 2, None);
    println!("{:?}, {}", s, s.point());
    let c = s.evolve_town();
    println!("{:?}, {}", c, c.point());
}

