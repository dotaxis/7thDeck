mod proton;

fn main() {
    let install_path = get_install_path();
    install_7th(&install_path);
    println!("yippee")
}

fn install_7th(install_path: &str) {
    let proton_versions = match proton::find_all_versions() {
        Ok(versions) => versions,
        Err(e) => {
            dialog_box::error(&e.to_string());
            panic!("{}", e);
        }
    };

    let args: Vec<String> = vec![
        "/VERYSILENT".to_string(),
        format!("/DIR=\"Z:{}\"", install_path),
        "/LOG=\"7thHeaven.log\"".to_string()
    ];

    let proton: &str = proton::find_highest_version(&proton_versions).unwrap().path.to_str().expect("Failed to get Proton");
    println!("Proton bin: {}", proton);
    proton::launch_exe_in_prefix(39140, "7th Heaven.exe", proton, &args);
}

fn get_install_path() -> String {
    "todo".to_string()
}
