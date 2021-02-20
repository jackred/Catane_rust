use enum_map::EnumMap;
use crate::resource::{ResourceDeck};

pub mod road;
pub mod town;
pub mod development_card;

pub trait Buyable {
    fn get_cost(&self) ->ResourceDeck;
}
