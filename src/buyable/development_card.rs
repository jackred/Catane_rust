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
    fn get_cost(&self) -> crate::resource::ResourceDeck {
        enum_map! {
            crate::resource::Resource::Grain => 1,
            crate::resource::Resource::Ore => 1,
            crate::resource::Resource::Wool => 1,
            _ => 0,
        }
    }
}
