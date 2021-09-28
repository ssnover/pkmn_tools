use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};

pub const PK4_SIZE: usize = 136;

pub struct Pokemon {
    pub species: u16,
}

impl Pokemon {
    pub fn from_bytes(data: &[u8; PK4_SIZE]) -> Option<Self> {
        let mut cursor = Cursor::new(data);
        cursor.seek(SeekFrom::Start(8)).unwrap();
        let species = cursor.read_u16::<LittleEndian>().unwrap();

        Some(Pokemon { species })
    }
}
