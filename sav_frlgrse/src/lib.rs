use byteorder::{LittleEndian, ReadBytesExt};
use num_derive::ToPrimitive;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Cursor, SeekFrom};
use std::path::Path;

pub const GAME_SAVE_DATA_LENGTH: usize = 131072;
const SAVE_INDEX_OFFSET: u64 = 0x0FFC;
const SAVE_A_OFFSET: u64 = 0x0000;
const SAVE_B_OFFSET: u64 = 0x00E000;
const SECTION_SIZE: u64 = 0x1000;

pub fn open_game_save<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = Vec::with_capacity(GAME_SAVE_DATA_LENGTH);
    reader.read_to_end(&mut contents)?;
    match contents.len() {
        GAME_SAVE_DATA_LENGTH => Ok(contents),
        _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameSave {
    A,
    B,
}

pub fn determine_latest_game_save(save: &[u8]) -> GameSave {
    let mut cursor = Cursor::new(save);
    cursor
        .seek(SeekFrom::Start(SAVE_A_OFFSET + SAVE_INDEX_OFFSET))
        .unwrap();
    let save_index_a = cursor.read_u32::<LittleEndian>().unwrap();

    cursor
        .seek(SeekFrom::Start(SAVE_B_OFFSET + SAVE_INDEX_OFFSET))
        .unwrap();
    let save_index_b = cursor.read_u32::<LittleEndian>().unwrap();

    if save_index_a == 0xffffffff {
        GameSave::B
    } else if save_index_b == 0xffffffff {
        GameSave::A
    } else {
        match save_index_a > save_index_b {
            true => GameSave::A,
            false => GameSave::B,
        }
    }
}

#[derive(ToPrimitive)]
pub enum Section {
    TrainerInfo = 0,
    TeamAndItems = 1,
    GameState = 2,
    MiscData = 3,
    RivalInfo = 4,
    PCBufferA = 5,
    PCBufferB = 6,
    PCBufferC = 7,
    PCBufferD = 8,
    PCBufferE = 9,
    PCBufferF = 10,
    PCBufferG = 11,
    PCBufferH = 12,
    PCBufferI = 13,
}

pub fn get_team_pokemon(save_data: &[u8]) -> Vec<Vec<u8>> {
    let save_offset = match determine_latest_game_save(save_data) {
        GameSave::A => SAVE_A_OFFSET,
        GameSave::B => SAVE_B_OFFSET,
    };

    let mut cursor = Cursor::new(save_data);
    cursor.seek(SeekFrom::Start(save_offset)).unwrap();
    cursor
        .seek(SeekFrom::Current(i64::try_from(SECTION_SIZE).unwrap() * 6))
        .unwrap();
    cursor.seek(SeekFrom::Current(0x0034)).unwrap();

    let team_size = cursor.read_u32::<LittleEndian>().unwrap();

    let mut read_buffer = [0u8; 100];

    let mut team_data = vec![];
    for _ in 0..team_size {
        cursor.read_exact(&mut read_buffer).unwrap();
        let mut pk3_data = Vec::with_capacity(100);
        pk3_data.extend_from_slice(&read_buffer[..]);
        team_data.push(pk3_data);
    }

    team_data
}
