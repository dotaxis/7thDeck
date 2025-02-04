mod steamhelper;

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use steamhelper::proton;
use steamhelper::game;
use dialog::DialogBox;
use sysinfo::System;

static FF7_APPID: u32 = 39140;

fn main() {
    let game = game::get_game(FF7_APPID).unwrap();
    steamhelper::kill_steam();
    // TODO: steamhelper::game::set_runner(FF7_APPID, proton9)
    steamhelper::game::wipe_prefix(&game);
    steamhelper::game::set_launch_options(&game).expect("Failed to set launch options");
    steamhelper::game::launch_game(&game).expect("Failed to launch FF7?");
    kill("FF7_Launcher");

    let exe_name = "7th_Heaven.exe";
    download_latest("tsunamods-codes/7th-Heaven", exe_name).expect("Failed to download 7th Heaven!");

    let install_path = get_install_path();
    dialog::Message::new(format!("Installing 7th Heaven to {:#?}", install_path.to_string_lossy()))
        .title("Path confirmed.")
        .show_with(dialog::backends::Dialog::new())
        .expect("Failed to display dialog box.");

    install_7th(exe_name, &install_path, "7thHeaven.log");
}

fn kill(pattern: &str){
    println!("Hello??");
    'kill: loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if process.name().contains(pattern) {
                println!("Found '{}' with PID: {}", pattern, pid);

                if process.kill() {
                    println!("Killed {} successfully.", pattern);
                    break 'kill;
                } else {
                    println!("Failed to kill {}!\nPlease exit {} manually and press Enter to continue.", pattern, pattern);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
    }
    println!("We made it out of the kill loop!");
}

fn install_7th(exe_path: &str, install_path: &Path, log_file: &str) {
    let install_path = install_path.to_str().unwrap().to_string();
    let proton_versions = proton::find_all_versions().expect("Failed to find any Proton versions!");

    let args: Vec<String> = vec![
        "/VERYSILENT".to_string(),
        format!("/DIR=Z:{}", install_path.replace('/', "\\")),
        format!("/LOG={}", log_file)
    ];

    let highest_proton_version = proton::find_highest_version(&proton_versions).unwrap();
    let proton = highest_proton_version.path.to_str().expect("Failed to get Proton").to_string();
    println!("Proton bin: {}", proton);

    let game = game::get_game(FF7_APPID).unwrap();

    match steamhelper::game::launch_exe_in_prefix(exe_path.into(), &game, &proton, &args) {
        Ok(_) => println!("Ran 7th Heaven installer"),
        Err(e) => panic!("{}", e)
    }
}

fn get_install_path() -> PathBuf {
    println!("Select an installation path for 7th Heaven.");
    loop {
        dialog::Message::new("Select an installation path for 7th Heaven.")
            .title("Select Destination")
            .show_with(dialog::backends::Dialog::new())
            .expect("Failed to display dialog box.");

        let install_path = match dialog::FileSelection::new("Select install path")
            .title("Folder Selection")
            .mode(dialog::FileSelectionMode::Open)
            .show_with(dialog::backends::Dialog::new())
            .expect("Failed to display file selection dialog box.") {
                Some(path) => path,
                None => {
                    println!("No path selected. Retrying.");
                    continue
                }
            };

        let confirmed = dialog::Question::new(format!("7th Heaven will be installed to:\n{:#?}\nConfirm?", install_path))
            .title("Confirm Install Location")
            .show_with(dialog::backends::Dialog::new())
            .expect("Failed to display dialog box.");

        if confirmed != dialog::Choice::Yes {
            println!("User did not confirm installation path. Retrying.");
            continue;
        }

        return PathBuf::from(install_path);
    }
}

fn download_latest(repo: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let release_url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let response: serde_json::Value = client
        .get(&release_url)
        .header("User-Agent", "rust-client")
        .send()?
        .json()?;

    let assets = response["assets"].as_array().ok_or("No assets found")?;
    let exe_asset = assets
        .iter()
        .find(|a| a["name"].as_str().unwrap_or("").ends_with(".exe"))
        .ok_or("No .exe asset found")?;

    let download_url = exe_asset["browser_download_url"]
        .as_str()
        .ok_or("No download URL")?;
    let exe_bytes = client.get(download_url).send()?.bytes()?;

    std::fs::write(output_path, exe_bytes)?;
    Ok(())
}
