use enum_map::EnumMap;
use crate::resource::{Resource, ResourceDeck};

pub mod road;
pub mod town;
pub mod development_card;

pub trait Buyable {
    fn get_cost() ->ResourceDeck;
}

#[derive(Debug, Enum)]
pub enum BuyableEnum {
    DevelopmentCard,
    Town,
    Settlement,
    Road,
}
