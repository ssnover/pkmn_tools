#![cfg(test)]

use std::io::Read;
use std::path::Path;

fn open_pk4<P: AsRef<Path>>(path: P) -> std::io::Result<[u8; pk4::PK4_SIZE]> {
    let mut pk4_file = std::fs::File::open(path)?;
    let mut pk4_data = [0u8; pk4::PK4_SIZE];
    pk4_file.read_exact(&mut pk4_data)?;
    Ok(pk4_data)
}

#[test]
fn verify_species() {
    let pk4_data = open_pk4("bins/pk4/squirtle.pk4").unwrap();
    let pkmn = pk4::Pokemon::from_bytes(&pk4_data).unwrap();
    assert_eq!(pkmn.species, 7);
}
