use seventh_deck::steamhelper;
use std::env;

static FF7_APPID: u32 = 39140;

fn main() {
    let launcher_bin = env::current_exe().expect("Failed to get binary path");
    let launcher_dir = launcher_bin.parent().expect("Failed to get binary directory");
    let seventh_heaven_exe = launcher_dir.join("7th Heaven.exe");

    if !seventh_heaven_exe.exists() {
        log::error!("Couldn't find '7th Heaven.exe'!");
        std::process::exit(1);
    }

    let game = steamhelper::game::get_game(FF7_APPID).unwrap();
    let proton_versions = steamhelper::proton::find_all_versions().expect("Failed to find any Proton versions!");
    let highest_proton_version = steamhelper::proton::find_highest_version(&proton_versions).unwrap();
    let proton = highest_proton_version.path.to_str().expect("Failed to get Proton").to_string();
    println!("Proton bin: {}", proton);

    steamhelper::game::launch_exe_in_prefix(seventh_heaven_exe, &game, &proton, None).expect("Failed to launch 7th Heaven.");
}
