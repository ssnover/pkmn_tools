use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u16)]
#[derive(Debug, FromPrimitive, PartialEq, ToPrimitive)]
pub enum Species {
    Squirtle = 7,
    Pidgey = 16,
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
