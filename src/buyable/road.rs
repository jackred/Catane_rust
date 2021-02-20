type CoordRoad =  (crate::Coord, crate::Coord);

#[derive(Debug)]
pub struct Road {
    location: CoordRoad,
    connected: Vec<Road>
}

impl Road {
    pub fn new(north_coord: crate::Coord, south_coord: crate::Coord) -> Road {
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
    fn get_cost(&self) -> crate::resource::ResourceDeck {
        enum_map! {
            crate::resource::Resource::Lumber => 1,
            crate::resource::Resource::Brick => 1,
            _ => 0,
        }
    }
}

