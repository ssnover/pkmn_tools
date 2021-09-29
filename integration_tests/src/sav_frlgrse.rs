#![cfg(test)]

use std::convert::TryInto;
use std::path::Path;

use pk3::PK3_SIZE;
use pkmn_utils::Species;

fn open_firered_sav<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let save_bin = sav_frlgrse::open_game_save(path)?;
    assert_eq!(save_bin.len(), sav_frlgrse::GAME_SAVE_DATA_LENGTH);
    Ok(save_bin)
}

#[test]
fn verify_parsing_party() {
    let save = open_firered_sav("bins/sav/firered_001.sav").unwrap();
    let team = sav_frlgrse::get_team_pokemon(&save);
    assert_eq!(team.len(), 2);

    let team: Vec<_> = team
        .into_iter()
        .map(|data| {
            pk3::Pokemon::from_bytes_with_encryption(&data[..PK3_SIZE].try_into().unwrap()).unwrap()
        })
        .collect();
    assert_eq!(team[0].species, Species::Squirtle);
    assert_eq!(team[1].species, Species::Pidgey);
}
