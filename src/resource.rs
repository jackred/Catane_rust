use enum_map::EnumMap;

#[derive(Debug, Enum)]
pub enum Resource {
    Grain,
    Lumber,
    Ore,
    Wool,
    Brick,
}

pub type ResourceDeck = EnumMap<Resource, i32>;
