use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};
use sysinfo::System;
use urlencoding::encode;

pub mod game;
pub mod proton;

pub fn get_library() -> Result<steamlocate::SteamDir> {
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
        let library = steamlocate::SteamDir::from_dir(libraries[0].as_path())
            .context("Couldn't get library")?;
        log::info!("Steam installation located: {}", libraries[0].display());
        return Ok(library);
    }

    log::warn!("Multiple Steam installations detected. Allowing user to select.");
    println!(
        "{} Multiple Steam installations detected.",
        console::style("!").yellow()
    );

    let choices = &[
        format!(
            "Native: {}",
            console::style(libraries[0].display()).bold().underlined()
        ),
        format!(
            "Flatpak: {}",
            console::style(libraries[1].display()).bold().underlined()
        ),
    ];
    let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Steam installation to continue:")
        .items(choices)
        .default(0)
        .interact()?;

    let library = steamlocate::SteamDir::from_dir(libraries[selection].as_path())
        .context("Failed to get library from dir")?;

    Ok(library)
}

pub fn kill_steam() -> Result<()> {
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut steam_running = false;

        for (pid, process) in sys.processes() {
            if process.name() == "steam" {
                steam_running = true;
                log::info!("Found 'steam' with PID: {pid}");
                if process.kill() {
                    log::info!("Killed Steam successfully.");
                    return Ok(());
                } else {
                    // todo: use dialoguer -- or should we leave this?
                    log::error!(
                        "Failed to kill Steam! Please exit Steam and press A or Enter to continue."
                    );
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    continue;
                }
            }
        }
        if !steam_running {
            log::warn!("I guess Steam isn't running. Continuing.");
            break;
        }
    }
    Ok(())
}

pub fn add_nonsteam_game(file: &Path, steam_dir: steamlocate::SteamDir) -> Result<()> {
    let file_dir = file
        .parent()
        .with_context(|| format!("Couldn't get parent of {file:?}"))?;
    let uid = users::get_current_uid();
    let mut tmp = PathBuf::from("/tmp");
    let mut steam_bin = "steam";

    // Flatpak Steam
    if steam_dir
        .path()
        .to_string_lossy()
        .contains("com.valvesoftware.Steam")
    {
        steam_bin = "flatpak run com.valvesoftware.Steam";
        tmp = PathBuf::from(format!(
            "/run/user/{uid}/.flatpak/com.valvesoftware.Steam/tmp"
        ));

        Command::new("flatpak")
            .args([
                "override",
                "--user",
                &format!("--filesystem={}", file_dir.display()),
                "com.valvesoftware.Steam",
            ])
            .status()?;

        kill_steam()?;
    }

    let encoded_url = format!(
        "steam://addnonsteamgame/{}",
        encode(&file.to_string_lossy())
    );
    let _ = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(tmp.join("addnonsteamgamefile"));

    Command::new(steam_bin)
        .arg(&encoded_url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    while Command::new("pgrep")
        .arg("steam")
        .stdout(Stdio::null())
        .status()
        .map(|s| !s.success())
        .unwrap_or(true)
    {
        sleep(Duration::from_secs(1));
    }

    log::info!("Added {file:?} to Steam!");
    Ok(())
}
