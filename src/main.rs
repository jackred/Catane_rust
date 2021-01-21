use std::io;
use rand::Rng;
// use std::collections::HashMap;

#[macro_use]
extern crate enum_map;
use enum_map::EnumMap;

#[derive(Debug, Enum)]
enum Resource {
    Grain,
    Lumber,
    Ore,
    Wool,
    Brick,
}



#[derive(Debug)]
enum TypeBuyable {
    DevelopmentCard,
    Settlement,
    City,
    Road
}

type ResourceDeck = EnumMap<Resource, i32>;



trait Buyable {
    fn get_cost() -> EnumMap<Resource, i32>;
}

mod development_card {
    type DevelopmentDeck =  super::EnumMap<DevelopmentCardValue, i32>;

    
    #[derive(Debug, Enum, Clone)]
    pub enum DevelopmentCardValue {
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
    
    fn knightAction() {
        println!("knight");
    }
    fn roadBuildingAction() {
        println!("road building");
    }
    fn yearOfPlentyAction() {
        println!("year of plenty");
    }
    fn monopolyAction() {
        println!("monopoly");
    }

    #[derive(Debug)]
    pub struct DevelopmentCard {
        value: DevelopmentCardValue,
        playable: bool,
        owned: Option<i32>
    }

    impl DevelopmentCard {
        #[inline]
        fn get_deck() -> DevelopmentDeck {
            enum_map! {
                DevelopmentCardValue::Knight => 14,
                DevelopmentCardValue::RoadBuilding | DevelopmentCardValue::YearOfPlenty | DevelopmentCardValue::Monopoly => 2,
                _ => 1,
            }
        }

        fn create_card(value: DevelopmentCardValue, playable:bool) -> DevelopmentCard {
            DevelopmentCard {
                playable: playable,
                value: value,
                owned: None
            }
        }
        
        pub fn create_deck() -> Vec<DevelopmentCard> {
            let deck = DevelopmentCard::get_deck();
            let mut res: Vec<DevelopmentCard> = vec![];
            for (key, &value) in &deck {
                for _ in 0..value {
                    res.push(DevelopmentCard::create_card(key.clone(), value>1));
                }
            }
            res
        }
        
        pub fn effect(&self) {
            match self.value {
                DevelopmentCardValue::Knight => knightAction(),
                DevelopmentCardValue::RoadBuilding => roadBuildingAction(),
                DevelopmentCardValue::YearOfPlenty => yearOfPlentyAction(),
                DevelopmentCardValue::Monopoly => monopolyAction(),
                _ => {}
            }
        }
    }

    impl super::Buyable for DevelopmentCard  {
        fn get_cost() -> super::ResourceDeck {
            enum_map! {
                super::Resource::Grain => 1,
                super::Resource::Ore => 1,
                super::Resource::Wool => 1,
                _ => 0,
            }
        }
    }
}

#[derive(Debug)]
struct Deck {
    resource_cards: ResourceDeck,
    research_cards:  Vec<development_card::DevelopmentCard>
}


// fn get_cost(buyable: TypeBuyable) -> ResourceDeck {
//     match buyable {
//         TypeBuyable::DevelopmentCard => enum_map! {
//             Resource::Grain => 1,
//             Resource::Ore => 1,
//             Resource::Wool => 1,
//             _ => 0,
//         },
//         TypeBuyable::Settlement =>  enum_map! {
//             Resource::Grain => 1,
//             Resource::Ore => 1,
//             Resource::Wool => 1,
//             Resource::Lumber => 1,
//             _ => 0,
//         },
//         TypeBuyable::City => enum_map! {
//             Resource::Grain => 2,
//             Resource::Ore => 3,
//             _ => 0,
//         },
//         TypeBuyable::Road => enum_map! {
//             Resource::Lumber =>1,
//             Resource::Brick => 1,
//             _ => 0,
//         }
//     }
// }





// #[derive(Debug)]
// struct Coord {
//     x: i32,
//     y: i32
// }

// #[derive(Debug)]
// enum Coords {
//     CoordTown(Coord),
//     CoordRoad(Coord, Coord)
// }


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
        Resource::Grain => 32,
        _ => 0,
    };
    println!("{:?}", map[Resource::Ore]);
    let deck = Deck {
        resource_cards: map,
        research_cards: development_card::DevelopmentCard::create_deck()
    };
    println!("{:?}", deck);
    //println!("City price: {:?}", get_cost(TypeBuyable::City));
    println!("Dev card: {:?}", development_card::DevelopmentCard::get_cost());
    
    
}

