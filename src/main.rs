mod steamhelper;

use std::path::PathBuf;

use steamhelper::proton;
use steamhelper::game;
use dialog::DialogBox;

fn main() {
    let install_path = get_install_path();
    //install_7th(&install_path);
    dialog::Message::new(format!("Installing 7th Heaven to {:#?}", install_path.to_string_lossy()))
        .title("Path confirmed.")
        .show()
        .expect("Failed to display dialog box.");
}

fn install_7th(install_path: &str) {
    let proton_versions = proton::find_all_versions().expect("Failed to find any Proton versions!");

    let args: Vec<String> = vec![
        "/VERYSILENT".to_string(),
        format!("/DIR=\"Z:{}\"", install_path),
        "/LOG=\"7thHeaven.log\"".to_string()
    ];

    let proton: &str = proton::find_highest_version(&proton_versions).unwrap().path.to_str().expect("Failed to get Proton");
    println!("Proton bin: {}", proton);

    let game = game::get_game(39140).unwrap();
    match steamhelper::launch_exe_in_prefix("7th Heaven.exe".into(), game, proton, &args) {
        Ok(_) => println!("Ran 7th Heaven installer"),
        Err(e) => panic!("{}", e)
    }
}

fn get_install_path() -> PathBuf {
    println!("Select an installation path for 7th Heaven.");
    loop {
        dialog::Message::new("Select an installation path for 7th Heaven.")
            .title("Select Destination")
            .show()
            .expect("Failed to display dialog box.");

        let install_path = match dialog::FileSelection::new("Select install path")
            .title("Folder Selection")
            .mode(dialog::FileSelectionMode::Save)
            .show()
            .expect("Failed to display file selection dialog box.") {
                Some(path) => path,
                None => {
                    println!("No path selected. Retrying.");
                    continue
                }
            };

        let confirmed = dialog::Question::new(format!("7th Heaven will be installed to:\n{:#?}\nConfirm?", install_path))
            .title("Confirm Install Location")
            .show()
            .expect("Failed to display dialog box.");

        if confirmed != dialog::Choice::Yes {
            println!("User did not confirm installation path. Retrying.");
            continue;
        }

        return PathBuf::from(install_path);
    }
}
