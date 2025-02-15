use seventh_deck::{logging, config_handler, steam_helper};
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

    let steam_dir_str = &config_handler::read_value("steam_dir");
    let steam_dir = steamlocate::SteamDir::from_dir(Path::new(steam_dir_str)).unwrap();
    log::info!("Steam path: {}", steam_dir.path().display());

    let game = steam_helper::game::get_game(FF7_APPID, steam_dir).unwrap();

    steam_helper::game::launch_exe_in_prefix(seventh_heaven_exe, &game, None).expect("Failed to launch 7th Heaven.");
}
