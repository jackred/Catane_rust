use std::io;
use rand::Rng;
// use std::collections::HashMap;
use core::fmt::Debug;

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
    fn get_cost(&self) -> EnumMap<Resource, i32>;
}

trait Town {
    fn point(&self) -> i32;
    fn resource_multiplier(&self) -> i32;
}

impl Debug for dyn Town {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Town{{{}}}", self.point())
    }
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
        player: Option<i32>
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

        fn new(value: DevelopmentCardValue, playable:bool) -> DevelopmentCard {
            DevelopmentCard {
                playable: playable,
                value: value,
                player: None
            }
        }
        
        pub fn create_deck() -> Vec<DevelopmentCard> {
            let deck = DevelopmentCard::get_deck();
            let mut res: Vec<DevelopmentCard> = vec![];
            for (key, &value) in &deck {
                for _ in 0..value {
                    res.push(DevelopmentCard::new(key.clone(), value>1));
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
        fn get_cost(&self) -> super::ResourceDeck {
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
        pub fn new(north_coord: super::Coord, south_coord: super::Coord) -> Road {
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
        fn get_cost(&self) -> super::ResourceDeck {
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
        pub fn new(location: super::Coord, player: i32, harbor: Option<bool>) -> Settlement {
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
            super::city::City::new(self.location, self.player, self.harbor)
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
        fn get_cost(&self) -> super::ResourceDeck {
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
        pub fn new(location: super::Coord, player: i32, harbor: Option<bool>) -> City{
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
        fn get_cost(&self) -> super::ResourceDeck {
            enum_map! {
                super::Resource::Ore => 3,
                super::Resource::Grain => 2,
                _ => 0,
            }
        }
    }
}

mod map {
    #[derive(Debug)]
    pub struct Map {
        thief: i32,
        tiles: Vec<super::tile::Tile>, // meant to be 2d array
        towns: Vec<Box<dyn super::Town>>, // meant to be 2d array
        roads: Vec<super::road::Road>, // meant to be 2d array
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
        resource: super::Resource,
        towns: Vec<Box<dyn super::Town>>,
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
        fn add_resources(&self, resources: super::ResourceDeck){}
        fn rm_resources(&self, resources: super::ResourceDeck){}
        fn gain_resources(&self, tiles: Vec<super::tile::Tile>){}
        fn buy(&self, buyable: Box<dyn super::Buyable>){} // maybe redo struct
        fn can_buy(&self, cost: super::ResourceDeck){}
        fn play_card(&self, card: super::development_card::DevelopmentCard){} // useless?
        fn trade(&self, gain:  super::ResourceDeck, traded:  super::ResourceDeck){}
        fn calculate_points(&self) -> i32{
            1
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
    deck.research_cards[0].effect();
    let r = road::Road::new(Coord{x:1,y:1}, Coord{x:2,y:2});
    println!("{:?}", r);
    let r2 = road::Road::new(Coord{x:1,y:0}, Coord{x:8,y:-8});
    println!("{:?}", r2);
    println!("{:?}", r.is_connected(&r2));
    println!("{:?}", r2.is_connected(&r));
    let s = settlement::Settlement::new(Coord{x: 5, y: 7}, 2, None);
    println!("{:?}, {}", s, s.point());
    let c = s.evolve_town();
    println!("{:?}, {}", c, c.point());
}

