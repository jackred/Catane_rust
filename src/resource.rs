use enum_map::EnumMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Enum, PartialEq, Eq, Copy, Clone)]
pub enum Resource {
    Grain,
    Lumber,
    Ore,
    Wool,
    Brick,
}

#[derive(Debug)]
pub struct ResourceDeck(pub EnumMap<Resource, i32>);

impl ResourceDeck {
    pub fn new(grain: i32, lumber: i32, ore: i32, wool: i32, brick: i32) -> ResourceDeck {
        ResourceDeck(enum_map! {
            Resource::Grain => grain,
            Resource::Lumber => lumber,
            Resource::Ore => ore,
            Resource::Wool => wool,
            Resource::Brick => brick,

        })
    }

    pub fn new_vec(vec: Vec<(Resource, i32)>) -> ResourceDeck{
        let mut map = enum_map! {_ => 0};
        for i in vec {
            map[i.0] = i.1;
        }
        ResourceDeck(map)
    }
}

impl Add for ResourceDeck {
    type Output = Self;

    fn add(mut self, other: Self) -> Self{
        for (key, value) in &mut self.0 {
            *value += other.0[key]
        }
        self
    }
}

impl Sub for ResourceDeck {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self{
        for (key, value) in &mut self.0 {
            *value -= other.0[key]
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

impl SubAssign for ResourceDeck {
    fn sub_assign(&mut self, other: Self){
        for (key, value) in &mut self.0 {
            *value -= other.0[key]
        }
    }
}
