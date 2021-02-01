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

trait Town {
    fn point(&self) -> i32;
    fn resource_multiplier(&self) -> i32;
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
    
    fn knight_action() {
        println!("knight");
    }
    fn road_building_action() {
        println!("road building");
    }
    fn year_of_plenty_action() {
        println!("year of plenty");
    }
    fn monopoly_action() {
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
                DevelopmentCardValue::Knight => knight_action(),
                DevelopmentCardValue::RoadBuilding => road_building_action(),
                DevelopmentCardValue::YearOfPlenty => year_of_plenty_action(),
                DevelopmentCardValue::Monopoly => monopoly_action(),
                _ => {}
            }
        }
    }

    impl super::Buyable for DevelopmentCard  {
        #[inline]
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


mod road {
    type CoordRoad =  (super::Coord, super::Coord);

    #[derive(Debug)]
    pub struct Road {
        location: CoordRoad,
        connected: Vec<Road>
    }

    impl Road {
        pub fn create_road(north_coord: super::Coord, south_coord: super::Coord) -> Road {
            Road {
                location: (north_coord, south_coord),
                connected: vec!()
            }
        }

        pub fn is_connected(&self, road: &Road) -> bool {
            self.location.0.is_connected(&road.location.0)
                ||  self.location.0.is_connected(&road.location.1)
                ||  self.location.1.is_connected(&road.location.0)
                ||  self.location.1.is_connected(&road.location.1)
        }
    }
    
    impl super::Buyable for Road  {
        #[inline]
        fn get_cost() -> super::ResourceDeck {
            enum_map! {
                super::Resource::Lumber => 1,
                super::Resource::Brick => 1,
                _ => 0,
            }
        }
    }
    
}

mod settlement {
    #[derive(Debug)]
    pub struct Settlement {
        location: super::Coord,
        player: i32, // player to be implemented
        harbor: Option<bool> // harbor type to be implemented
    }

    impl Settlement {
        pub fn create_settlement(location: super::Coord, player: i32, harbor: Option<bool>) -> Settlement {
            Settlement {
                location: location,
                player: player,
                harbor: harbor
            }
        }
        
        pub fn get_player(&self) -> i32 {
            self.player
        }

        pub fn evolve_town(self) -> super::city::City {
            super::city::City::create_city(self.location, self.player, self.harbor)
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

    impl super::Buyable for Settlement  {
        #[inline]
        fn get_cost() -> super::ResourceDeck {
            enum_map! {
                super::Resource::Lumber => 1,
                super::Resource::Brick => 1,
                super::Resource::Wool => 1,
                super::Resource::Grain => 1,
                _ => 0,
            }
        }
    }
}

mod city {
    #[derive(Debug)]
    pub struct City {
        location: super::Coord,
        player: i32, // player to be implemented
        harbor: Option<bool> // harbor type to be implemented
    }

    impl City {
        pub fn create_city(location: super::Coord, player: i32, harbor: Option<bool>) -> City{
            City {
                location: location,
                player: player,
                harbor: harbor
            }
        }
        
        pub fn get_player(&self) -> i32 {
            self.player
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
    }

    impl super::Buyable for City  {
        #[inline]
        fn get_cost() -> super::ResourceDeck {
            enum_map! {
                super::Resource::Ore => 3,
                super::Resource::Grain => 2,
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
    //println!("Town price: {:?}", get_cost(TypeBuyable::Town));
    println!("Dev card: {:?}", development_card::DevelopmentCard::get_cost());
    deck.research_cards[0].effect();
    let r = road::Road::create_road(Coord{x:1,y:1}, Coord{x:2,y:2});
    println!("{:?}", r);
    let r2 = road::Road::create_road(Coord{x:1,y:0}, Coord{x:8,y:-8});
    println!("{:?}", r2);
    println!("{:?}", r.is_connected(&r2));
    println!("{:?}", r2.is_connected(&r));
    let s = settlement::Settlement::create_settlement(Coord{x: 5, y: 7}, 2, None);
    println!("{:?}, {}", s, s.point());
    let c = s.evolve_town();
    println!("{:?}, {}", c, c.point());
}

