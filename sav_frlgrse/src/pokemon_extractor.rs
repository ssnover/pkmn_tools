use clap::Clap;
use std::convert::TryInto;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(name = "frlgrse_pokemon_extractor")]
struct Args {
    #[clap(long)]
    input: PathBuf,

    #[clap(long)]
    output_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input_save_file = args.input;
    let save_data = sav_frlgrse::open_game_save(input_save_file)?;
    let mut team = sav_frlgrse::get_team_pokemon(&save_data);

    if args.output_dir.exists() && !args.output_dir.is_dir() {
        eprintln!(
            "{} is not a directory",
            args.output_dir.as_os_str().to_string_lossy()
        );
        std::process::exit(1);
    }

    if !args.output_dir.exists() {
        std::fs::create_dir_all(&args.output_dir)?;
    }

    for (idx, pkmn) in team.iter_mut().enumerate() {
        if pkmn.len() == pk3::PK3_SIZE {
            pk3::decrypt_pokemon(pkmn);
            let mut file_path = args.output_dir.clone();
            let pokemon = pk3::Pokemon::from_bytes(pkmn[..pk3::PK3_SIZE].try_into()?).unwrap();
            let pokemon_name = pokemon.species.to_string().to_ascii_lowercase();
            file_path.push(format!("team_{}_{}.pk3", idx, pokemon_name));
            let mut pkmn_file = std::fs::File::create(file_path)?;
            pkmn_file.write_all(&pkmn[..])?;
        }
    }

    Ok(())
}
