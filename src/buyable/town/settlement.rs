#[derive(Debug)]
pub struct Settlement {
    location: crate::Coord,
    player: i32, // player to be implemented
    harbor: Option<bool> // harbor type to be implemented
}

impl Settlement {
    pub fn new(location: crate::Coord, player: i32, harbor: Option<bool>) -> Settlement {
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

impl super::super::Buyable for Settlement  {
    #[inline]
    fn get_cost(&self) -> crate::resource::ResourceDeck {
        enum_map! {
            crate::resource::Resource::Lumber => 1,
            crate::resource::Resource::Brick => 1,
            crate::resource::Resource::Wool => 1,
            crate::resource::Resource::Grain => 1,
            _ => 0,
        }
    }
}
