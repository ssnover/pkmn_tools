use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::convert::TryInto;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};

use pkmn_utils::Species;

pub mod held_item;
pub use held_item::HeldItem;

pub const PK3_SIZE: usize = 100;

pub struct Stats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub special_attack: u16,
    pub special_defense: u16,
}

pub struct ContestStats {
    pub coolness: u8,
    pub beauty: u8,
    pub cuteness: u8,
    pub smartness: u8,
    pub toughness: u8,
    pub feel: u8,
}

pub struct PokerusStatus {
    pub days_left: u8,
    pub strain: u8,
}

pub struct Pokemon {
    pub personality_value: u32,
    pub original_trainer_id: u32,
    pub nickname: [u8; 10],
    pub language: Language,
    pub original_trainer_name: [u8; 7],
    pub markings: u8,
    pub species: Species,
    pub level: u8,
    pub friendship: u8,
    pub experience: u32,
    pub pp_bonuses: u8,
    pub held_item: Option<HeldItem>,
    pub moves: [u16; 4],
    pub power_points: [u8; 4],
    pub effort_values: Stats,
    pub contest_stats: ContestStats,
    pub pokerus: PokerusStatus,
}

impl Pokemon {
    pub fn from_bytes(data: &[u8; PK3_SIZE]) -> Option<Self> {
        Self::from_bytes_impl(data, false)
    }

    pub fn from_bytes_with_encryption(data: &[u8; PK3_SIZE]) -> Option<Self> {
        Self::from_bytes_impl(data, true)
    }

    fn from_bytes_impl(data: &[u8; PK3_SIZE], should_decrypt: bool) -> Option<Self> {
        let mut vec_data = vec![];
        vec_data.extend_from_slice(data);
        if should_decrypt {
            decrypt_pokemon(&mut vec_data);
        }

        let mut cursor = Cursor::new(&vec_data);
        let personality_value = cursor.read_u32::<LittleEndian>().unwrap();
        let original_trainer_id = cursor.read_u32::<LittleEndian>().unwrap();
        
        // TODO: Try parsing the buffer in some way to be readable
        let mut nickname_buffer = [0u8; 10];
        cursor.read_exact(&mut nickname_buffer).unwrap();

        let language = FromPrimitive::from_u16(cursor.read_u16::<LittleEndian>().unwrap()).unwrap();

        // TODO: Try parsing into a string?
        let mut ot_name_buffer = [0u8; 7];
        cursor.read_exact(&mut ot_name_buffer).unwrap();

        let markings = cursor.read_u8().unwrap();

        cursor.seek(SeekFrom::Start(32)).unwrap();
        let species = cursor.read_u16::<LittleEndian>().unwrap();
        let species = FromPrimitive::from_u16(species).unwrap();

        let held_item = match cursor.read_u16::<LittleEndian>().unwrap() {
            0 => None,
            n => Some(FromPrimitive::from_u16(n).unwrap()),
        };

        let experience = cursor.read_u32::<LittleEndian>().unwrap();
        let pp_bonuses = cursor.read_u8().unwrap();
        let friendship = cursor.read_u8().unwrap();
        let _ = cursor.read_u16::<LittleEndian>().unwrap();

        let moves: Vec<_> = (0..4).into_iter().map(|_i| cursor.read_u16::<LittleEndian>().unwrap()).collect();
        let moves: [u16; 4] = moves[0..4].try_into().unwrap();
        let power_points: Vec<_> = (0..4).into_iter().map(|_i| cursor.read_u8().unwrap()).collect();
        let power_points: [u8; 4] = power_points[0..4].try_into().unwrap();

        let evs = Stats {
            hp: cursor.read_u8().unwrap().into(),
            attack: cursor.read_u8().unwrap().into(),
            defense: cursor.read_u8().unwrap().into(),
            speed: cursor.read_u8().unwrap().into(),
            special_attack: cursor.read_u8().unwrap().into(),
            special_defense: cursor.read_u8().unwrap().into(),
        };

        let contest_stats = ContestStats {
            coolness: cursor.read_u8().unwrap(),
            beauty: cursor.read_u8().unwrap(),
            cuteness: cursor.read_u8().unwrap(),
            smartness: cursor.read_u8().unwrap(),
            toughness: cursor.read_u8().unwrap(),
            feel: cursor.read_u8().unwrap(),
        };

        let pokerus_data = cursor.read_u8().unwrap();
        let pokerus = PokerusStatus {
            days_left: pokerus_data & 0x0f,
            strain: (pokerus_data & 0xf0) >> 4,
        };

        cursor.seek(SeekFrom::Start(84)).unwrap();
        let level = cursor.read_u8().unwrap();

        Some(Pokemon {
            personality_value,
            original_trainer_id,
            nickname: nickname_buffer,
            language,
            original_trainer_name: ot_name_buffer,
            markings,
            species,
            level,
            friendship,
            experience,
            pp_bonuses,
            held_item,
            moves,
            power_points,
            effort_values: evs,
            contest_stats,
            pokerus,
        })
    }
}

pub fn decrypt_pokemon(pk3_data: &mut Vec<u8>) {
    let mut cursor = Cursor::new(&pk3_data);
    let personality_value = cursor.read_u32::<LittleEndian>().unwrap();
    let original_trainer_id = cursor.read_u32::<LittleEndian>().unwrap();
    let decryption_key = personality_value ^ original_trainer_id;
    let mut decryption_key_buf = [0u8; 4];
    LittleEndian::write_u32(&mut decryption_key_buf, decryption_key);

    // First XOR the key over the region
    for idx in (32..80).step_by(4) {
        for byte in 0..4 {
            pk3_data[idx + byte] ^= decryption_key_buf[byte];
        }
    }

    // Now rearrange the elements
    let mut rearranged_data = Vec::with_capacity(48);
    // Growth
    match personality_value % 24 {
        0..=5 => rearranged_data.extend_from_slice(&pk3_data[32..44]),
        6 | 7 | 12 | 13 | 18 | 19 => rearranged_data.extend_from_slice(&pk3_data[44..56]),
        8 | 10 | 14 | 16 | 20 | 22 => rearranged_data.extend_from_slice(&pk3_data[56..68]),
        9 | 11 | 15 | 17 | 21 | 23 => rearranged_data.extend_from_slice(&pk3_data[68..80]),
        24u32..=u32::MAX => unreachable!(),
    };

    // Attacks
    match personality_value % 24 {
        6..=11 => rearranged_data.extend_from_slice(&pk3_data[32..44]),
        0 | 1 | 14 | 15 | 20 | 21 => rearranged_data.extend_from_slice(&pk3_data[44..56]),
        2 | 4 | 12 | 17 | 18 | 23 => rearranged_data.extend_from_slice(&pk3_data[56..68]),
        3 | 5 | 13 | 16 | 19 | 22 => rearranged_data.extend_from_slice(&pk3_data[68..80]),
        _ => unimplemented!(),
    };

    // EVs and Condition
    match personality_value % 24 {
        12..=17 => rearranged_data.extend_from_slice(&pk3_data[32..44]),
        2 | 3 | 8 | 9 | 22 | 23 => rearranged_data.extend_from_slice(&pk3_data[44..56]),
        0 | 5 | 6 | 11 | 19 | 21 => rearranged_data.extend_from_slice(&pk3_data[56..68]),
        1 | 4 | 7 | 10 | 18 | 20 => rearranged_data.extend_from_slice(&pk3_data[68..80]),
        _ => unimplemented!(),
    };

    // Miscellaneous
    match personality_value % 24 {
        18..=23 => rearranged_data.extend_from_slice(&pk3_data[32..44]),
        4 | 5 | 10 | 11 | 16 | 17 => rearranged_data.extend_from_slice(&pk3_data[44..56]),
        1 | 3 | 7 | 9 | 13 | 15 => rearranged_data.extend_from_slice(&pk3_data[56..68]),
        0 | 2 | 6 | 8 | 12 | 14 => rearranged_data.extend_from_slice(&pk3_data[68..80]),
        _ => unimplemented!(),
    };

    pk3_data[32..80].clone_from_slice(&rearranged_data[..(80 - 32)])
}

#[repr(u16)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Language {
    Japanese = 0x0201,
    English = 0x0202,
    French = 0x0203,
    Italian = 0x0204,
    German = 0x0205,
    Korean = 0x0206,
    Spanish = 0x0207,
}
