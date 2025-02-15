use std::{error::Error, io, path::PathBuf};
use dialoguer::theme::ColorfulTheme;
use sysinfo::System;

pub mod game;
pub mod proton;

pub fn get_library() -> Result<steamlocate::SteamDir, Box<dyn Error>> {
    let home_dir = home::home_dir().expect("Couldn't get $HOME?");
    let possible_libraries = vec![
        // Native install directory
        home_dir.join(".steam/root"),
        // Flatpak install directory
        home_dir.join(".var/app/com.valvesoftware.Steam/.steam/root"),
    ];

    let libraries: Vec<PathBuf> = possible_libraries
        .into_iter()
        .filter(|path| path.exists())
        .collect();

    if libraries.len() == 1 {
        let library = steamlocate::SteamDir::from_dir(libraries[0].as_path()).unwrap();
        log::info!("Steam installation located: {}", libraries[0].display());
        return Ok(library);
    }

    log::warn!("Multiple Steam installations detected. Allowing user to select.");
    println!("{} Multiple Steam installations detected.", console::style("!").yellow());

    let choices = &[
        format!("Native: {}", console::style(libraries[0].display()).bold().underlined()),
        format!("Flatpak: {}", console::style(libraries[1].display()).bold().underlined()),
    ];
    let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Steam installation to continue:")
        .items(choices)
        .default(0)
        .interact()?;

    Ok(steamlocate::SteamDir::from_dir(libraries[selection].as_path()).unwrap())
}

pub fn kill_steam() {
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut steam_running = false;

        for (pid, process) in sys.processes() {
            if process.name() == "steam" {
                steam_running = true;
                log::info!("Found 'steam' with PID: {}", pid);
                if process.kill() {
                    log::info!("Killed Steam successfully.");
                    return;
                } else {
                    // todo: use dialoguer -- or should we leave this?
                    log::error!("Failed to kill Steam! Please exit Steam and press A or Enter to continue.");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
        if !steam_running {
            log::warn!("I guess Steam isn't running. Continuing.");
            break;
        }
    }
}
