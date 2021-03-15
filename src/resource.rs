use enum_map::EnumMap;

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

impl ResourceDeck {
    pub fn add(&mut self, other: ResourceDeck) {
        for (key, value) in &mut self.0 {
            *value += other.0[key]
        }
    }
}
