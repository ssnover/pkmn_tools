#![cfg(test)]

use std::io::Read;
use std::path::Path;

use pkmn_utils::Species;

fn open_pk3<P: AsRef<Path>>(path: P) -> std::io::Result<[u8; pk3::PK3_SIZE]> {
    let mut pk3_file = std::fs::File::open(path)?;
    let mut pk3_data = [0u8; pk3::PK3_SIZE];
    pk3_file.read_exact(&mut pk3_data)?;
    Ok(pk3_data)
}

#[test]
fn verify_level() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.level, 7);
}

#[test]
fn verify_species() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.species, Species::Squirtle);
}

#[test]
fn verify_happiness() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.friendship, 80);
}

#[test]
fn verify_experience() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.experience, 293);
}

#[test]
fn verify_no_held_item() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.held_item, None);
}

#[test]
fn verify_personality_value() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.personality_value, 2368673094);
}

#[test]
fn verify_ot_id() {
    let pk3_data = open_pk3("bins/pk3/squirtle.pk3").unwrap();
    let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
    assert_eq!(pkmn.original_trainer_id, 1798070812);
}
