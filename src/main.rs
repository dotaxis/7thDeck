mod steamhelper;
mod dialog;

use steamhelper::proton;
use steamhelper::game;

fn main() {
    let install_path = get_install_path();
    let dialog_command = dialog::which_dialog();
    //install_7th(&install_path);
    dialog::prompt_ok(&dialog_command, format!("{:?}", game::get_game(39140).unwrap()));
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

fn get_install_path() -> String {
    "todo".to_string()
}
