use seventh_deck::{logging, steam_helper};
use std::{env, path::Path};

static FF7_APPID: u32 = 39140;

fn main() {
    logging::init();

    let launcher_bin = env::current_exe().expect("Failed to get binary path");
    let launcher_dir = launcher_bin.parent().expect("Failed to get binary directory");
    let seventh_heaven_exe = launcher_dir.join("7th Heaven.exe");

    if !seventh_heaven_exe.exists() {
        panic!("Couldn't find '7th Heaven.exe'!");
    }

    let toml_path = launcher_dir.join("7thDeck.toml");
    log::info!("TOML path: {}", toml_path.display());

    let toml_string = std::fs::read_to_string(toml_path).expect("Couldn't read the TOML file");
    let toml_value: toml::Value = toml::from_str(&toml_string).expect("Couldn't deserialize TOML");
    let steam_dir_str = toml_value.get("steam_dir")
        .expect("Couldn't find steam_dir key")
        .as_str()
        .expect("steam_dir value is not a string");
    let steam_dir = steamlocate::SteamDir::from_dir(Path::new(steam_dir_str)).unwrap();
    log::info!("Steam path: {}", steam_dir.path().display());

    let game = steam_helper::game::get_game(FF7_APPID, steam_dir).unwrap();
    let proton_versions = steam_helper::proton::find_all_versions().expect("Failed to find any Proton versions!");
    let highest_proton_version = steam_helper::proton::find_highest_version(&proton_versions).unwrap();
    let proton = highest_proton_version.path.to_str().expect("Failed to get Proton").to_string();
    log::info!("Proton bin: {}", proton);

    steam_helper::game::launch_exe_in_prefix(seventh_heaven_exe, &game, &proton, None).expect("Failed to launch 7th Heaven.");
}
