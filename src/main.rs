mod steamhelper;

use std::path::{Path, PathBuf};

use steamhelper::proton;
use steamhelper::game;
use dialog::DialogBox;
use sysinfo::System;

static FF7_APPID: u32 = 39140;

fn main() {
    let game = game::get_game(FF7_APPID).unwrap();
    let install_path = get_install_path();
    dialog::Message::new(format!("Installing 7th Heaven to {:#?}", install_path.to_string_lossy()))
        .title("Path confirmed.")
        .show_with(dialog::backends::Dialog::new())
        .expect("Failed to display dialog box.");
    steamhelper::kill_steam();
    // TODO: steamhelper::game::set_runner(FF7_APPID, proton9)
    steamhelper::game::wipe_prefix(&game);
    steamhelper::game::set_launch_options(&game).expect("Failed to set launch options");
    steamhelper::game::launch_game(&game).expect("Failed to launch FF7?");
    kill_ff7();
    install_7th(&install_path, "7thHeaven.log");
}

fn kill_ff7(){
    println!("Hello??");
    'kill: loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if process.name().contains("FF7_Launcher") {
                println!("Found 'FF7_Launcher' with PID: {}", pid);

                if process.kill() {
                    println!("Killed FF7 successfully.");
                    break 'kill;
                } else {
                    println!("Failed to kill FF7! Please exit the FF7 Launcher and press Enter to continue.");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
    }
    println!("We made it out of the kill loop!");
}

fn install_7th(install_path: &Path, log_file: &str) {
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

    match steamhelper::game::launch_exe_in_prefix("7th Heaven.exe".into(), game, &proton, &args) {
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
