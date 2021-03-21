pub mod city;
pub mod settlement;

pub trait Town {
    fn point(&self) -> i32;
    fn resource_multiplier(&self) -> i32;
    fn get_player(&self) -> i32;
}

impl core::fmt::Debug for dyn Town {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Town{{{}}}", self.point())
    }
}
