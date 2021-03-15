use enum_map::EnumMap;
use std::ops::{Add, AddAssign};

#[derive(Debug, Enum)]
pub enum Resource {
    Grain,
    Lumber,
    Ore,
    Wool,
    Brick,
}

#[derive(Debug)]
pub struct ResourceDeck(pub EnumMap<Resource, i32>);

impl Add for ResourceDeck {
    type Output = Self;

    fn add(mut self, other: Self) -> Self{
        for (key, value) in &mut self.0 {
            *value += other.0[key]
        }
        self
    }
}

impl AddAssign for ResourceDeck {
    fn add_assign(&mut self, other: Self){
        for (key, value) in &mut self.0 {
            *value += other.0[key]
        }
    }
}
