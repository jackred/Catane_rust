use enum_map::EnumMap;


pub mod road;
pub mod town;
pub mod development_card;

pub trait Buyable {
    fn get_cost(&self) -> EnumMap<super::resource::Resource, i32>;
}
