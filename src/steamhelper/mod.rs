use std::{io, error::Error, path::PathBuf, process::Command};
use sysinfo::{ProcessExt, Signal, System, SystemExt};

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

pub fn kill_steam() {
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if process.name() == "steam" {
                println!("Found 'steam' with PID: {}", pid);

                if process.kill() {
                    println!("Killed Steam successfully.");
                    break;
                } else {
                    println!("Failed to kill Steam! Please exit Steam and press Enter to continue.");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
    }
}
