#![cfg(test)]
mod tests {
    use std::io::Read;
    use std::path::Path;

    fn open_pk3<P: AsRef<Path>>(path: &P) -> std::io::Result<[u8; pk3::PK3_SIZE]> {
        let mut pk3_file = std::fs::File::open(path)?;
        let mut pk3_data = [0u8; pk3::PK3_SIZE];
        pk3_file.read_exact(&mut pk3_data)?;
        Ok(pk3_data)
    }

    #[test]
    fn verify_level() {
        let pk3_data = open_pk3(&"bins/pk3/squirtle.pk3").unwrap();
        let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
        assert_eq!(pkmn.level, 7);
    }

    #[test]
    fn verify_species() {
        let pk3_data = open_pk3(&"bins/pk3/squirtle.pk3").unwrap();
        let pkmn = pk3::Pokemon::from_bytes(&pk3_data).unwrap();
        assert_eq!(pkmn.species, 7);
    }

    fn open_firered_sav<P: AsRef<Path>>(path: &P) -> std::io::Result<Vec<u8>> {
        let save_bin = frlgrse_sav::open_game_save(path)?;
        assert_eq!(save_bin.len(), frlgrse_sav::GAME_SAVE_DATA_LENGTH);
        Ok(save_bin)
    }

    #[test]
    fn verify_parsing_party_size() {
        let save = open_firered_sav(&"bins/sav/firered_001.sav").unwrap();
        let team = frlgrse_sav::get_team_pokemon(&save);
        assert_eq!(team.len(), 2);
    }
}
