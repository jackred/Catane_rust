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

pub fn add_resource(map: &mut ResourceDeck, to_add: ResourceDeck) {
    for (key, value) in map {
        *value += to_add[key]
    }
}
