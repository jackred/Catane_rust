#![allow(dead_code)]
use std::io;
// use std::collections::HashMap;

#[macro_use]
extern crate enum_map;


mod resource;
mod buyable;
mod utility;
mod player;
mod map;

use utility::{Coord, Deck, roll_6_dices};
use buyable::Buyable;
use buyable::town::Town;
use buyable::town::city::City;


fn main() {
    println!("Please, enter your name: ");
    let mut name = String::new(); 
    io::stdin().read_line(&mut name)
        .expect("Failed to read the line");
    println!("Your name is : {}", name);
    println!("Rolling the dices...");
    let (d1, d2) = roll_6_dices();
    println!("Dice 1: {}, Dice 2: {}", d1, d2);
    let mut map = resource::ResourceDeck(enum_map! {
        resource::Resource::Grain => 32,
        _ => 0,
    });
    //println!("{:?}", map+map2);
    let deck = Deck {
        resource_cards: map,
        research_cards: buyable::development_card::DevelopmentCard::create_deck()
    };
    println!("{:?}", deck);
    println!("Town price: {:?}", City::get_cost());
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
    println!("----");
    println!("----");
    println!("----");
    let mut map = resource::ResourceDeck(enum_map! {
        resource::Resource::Grain => 32,
        _ => 0,
    });
    let map2 = resource::ResourceDeck(enum_map! {
        resource::Resource::Grain => 4,
        resource::Resource::Lumber => 1,
        _ => 0,
    });
    println!("{:?}", map);
    map.add(map2);
    println!("{:?}", map);
    println!("{:?}", map.0[resource::Resource::Ore]);
    //println!("{:?}", map2.0[resource::Resource::Lumber]);
}

