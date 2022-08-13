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
use map::tile;
use buyable::town::Town;
use buyable::road::Road;
use buyable::town::city::City;
use buyable::town::settlement::Settlement;


fn test() {
    println!("test!");
    let mut my_resource = resource::ResourceDeck(enum_map! {
        resource::Resource::Grain => 3,
	resource::Resource::Lumber => 1,
	resource::Resource::Ore => 2,
	resource::Resource::Brick => 1,
        _ => 0,
    });
    println!("Can I buy a city? {}", my_resource > City::get_cost());
    println!("Can I buy a settlement? {}", my_resource > Settlement::get_cost());
    println!("Can I buy a road? {}", my_resource > Road::get_cost());
    println!("end test!\n\n");
}

fn main() {
    test();
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
    let r = buyable::road::Road::new(Coord{x:1,y:1}, Coord{x:2,y:2}, 1);
    println!("{:?}", r);
    let r2 = buyable::road::Road::new(Coord{x:1,y:0}, Coord{x:8,y:-8}, 1);
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
    let map3 = resource::ResourceDeck(enum_map! {
        resource::Resource::Grain => 9,
        resource::Resource::Lumber => 12,
        _ => 0,
    });
    println!("{:?}", map);
    map = map - map2;
    println!("{:?}", map);
    println!("{:?}", map);
    println!("{:?}", map.0[resource::Resource::Ore]);
    //println!("{:?}", map2.0[resource::Resource::Lumber]);
    println!("----");
    println!("----");
    let mut player = player::Player{name: "Dorian".to_string(), cards: deck, buildings: vec![]};
    println!("{:?}", player);
    println!("{:?}", player.cards.resource_cards);
    player.add_resources(resource::ResourceDeck(enum_map! {
        resource::Resource::Grain => 9,
        resource::Resource::Lumber => 12,
        _ => 0,
    }));
    println!("{:?}", player.cards.resource_cards);
    let teci: Box<dyn buyable::town::Town> = Box::new(buyable::town::city::City::new(Coord{x: 5, y: 7}, 1, None));
    let tile = map::tile::Tile{hex: tile::TileType::Field, towns: vec![teci], coord: Coord{x: 5, y: 7}, number: 2};
    player.gain_resources(vec![]);
    println!("{:?}", player.cards.resource_cards);
    player.gain_resources(vec![tile]);
    println!("{:?}", player.cards.resource_cards);
    let res = resource::Resource::Grain;
    println!("res: {:?}", res);
    //println!("{:?}", map.0[res]);
    println!("{:?}", enum_map! {
        resource::Resource::Grain if res == resource::Resource::Grain => 9,
        resource::Resource::Lumber if res == resource::Resource::Lumber => 12,
        _ => 0,
    });
    println!("{:?}", resource::ResourceDeck::new_vec(vec![(resource::Resource::Grain, 4)]));
    let teci: Box<dyn buyable::town::Town> = Box::new(buyable::town::city::City::new(Coord{x: 5, y: 7}, 2, None));
    println!("-----");
    println!("{:?}", teci.resource_multiplier());
}

