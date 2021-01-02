use std::io;
use rand::Rng;
// use std::collections::HashMap;

#[macro_use]
extern crate enum_map;
use enum_map::EnumMap;

#[derive(Debug, Enum)]
enum Resource {
    Wheat,
    Wood,
    Stone,
    Sheep,
    Clay,
}

#[derive(Debug, Enum)]
enum ResearchCardValue {
    Knight,
    RoadBuilding,
    YearOfPlenty,
    Monopoly,
    University,
    Market,
    GreatHall,
    Chapel,
    Library,
}

#[derive(Debug)]
struct Deck {
    resource_cards: EnumMap<Resource, i32>,
    research_cards: EnumMap<Resource, i32>,
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
        Resource::Wheat => 32,
        _ => 0,
    };
    println!("{:?}", map[Resource::Stone]);
    let deck = Deck {
        resource_cards: map,
        research_cards:  enum_map! {
            _ => 0,
        }
    };
    println!("{:?}", deck);
}

