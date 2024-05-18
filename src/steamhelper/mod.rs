use std::{error::Error, path::PathBuf, process::Command};

pub mod game;
pub mod proton;


pub fn launch_exe_in_prefix(exe_to_launch: PathBuf, game: game::SteamGame, proton_path: &str, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new(proton_path);
    command
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", game.client_path)
        .env("STEAM_COMPAT_DATA_PATH", game.prefix.as_path())
        .env("WINEDLLOVERRIDES", "dinput.dll=n,b")
        .arg("run")
        .arg(&exe_to_launch);
    for arg in args { command.arg(arg); }
    command.spawn()?.wait()?;

    Ok(println!("Launched {}", exe_to_launch.file_name().unwrap().to_string_lossy()))
}
