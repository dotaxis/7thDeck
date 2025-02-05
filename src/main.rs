mod steamhelper;

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use steamhelper::proton;
use steamhelper::game;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use sysinfo::System;

static FF7_APPID: u32 = 39140;

fn main() {
    let game = game::get_game(FF7_APPID).unwrap();
    steamhelper::kill_steam();
    steamhelper::game::set_runner(&game, "proton_9").expect("Failed to set runner"); // TODO: Expand this to allow Proton version selection
    steamhelper::game::wipe_prefix(&game).expect("Failed to wipe prefix");
    steamhelper::game::set_launch_options(&game).expect("Failed to set launch options");
    steamhelper::game::launch_game(&game).expect("Failed to launch FF7?");
    kill("FF7_Launcher");

    let exe_name = "7th_Heaven.exe";
    download_latest("tsunamods-codes/7th-Heaven", exe_name).expect("Failed to download 7th Heaven!");

    let install_path = get_install_path();
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Path confirmed.")
        .set_text(&format!("Installing 7th Heaven to {:#?}", install_path.to_string_lossy()))
        .show_alert()
        .unwrap();

    install_7th(exe_name, &install_path, "7thHeaven.log");
}

fn kill(pattern: &str){
    println!("Waiting for prefix to rebuild.");
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
        MessageDialog::new()
            .set_text("Select an installation path for 7th Heaven.")
            .set_title("Select Destination")
            .show_alert()
            .unwrap();

        let install_path = match FileDialog::new()
            .set_location("~")
            .set_title("Select Destination")
            .show_open_single_dir()
            .unwrap() {
                Some(path) => path,
                None => {
                    println!("No path selected. Retrying.");
                    continue
                }
            };

        let confirmed = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Confirm Install Location")
            .set_text(&format!("7th Heaven will be installed to:\n{:#?}\nConfirm?", install_path))
            .show_confirm()
            .unwrap();

        if !confirmed {
            println!("User did not confirm installation path. Retrying.");
            continue;
        }

        return install_path;
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
