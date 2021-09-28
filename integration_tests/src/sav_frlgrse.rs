#![cfg(test)]

use std::path::Path;

fn open_firered_sav<P: AsRef<Path>>(path: &P) -> std::io::Result<Vec<u8>> {
    let save_bin = sav_frlgrse::open_game_save(path)?;
    assert_eq!(save_bin.len(), sav_frlgrse::GAME_SAVE_DATA_LENGTH);
    Ok(save_bin)
}

#[test]
fn verify_parsing_party_size() {
    let save = open_firered_sav(&"bins/sav/firered_001.sav").unwrap();
    let team = sav_frlgrse::get_team_pokemon(&save);
    assert_eq!(team.len(), 2);
}
