mod proton;

fn main() {
    let ff7_id: u32 = 39140;
    println!("{}", proton::find_prefix(ff7_id).unwrap().display());

    let proton_versions = match proton::find_all_versions() {
        Ok(versions) => versions,
        Err(e) => {
            dialog_box::error(&e.to_string());
            panic!("{}", e);
        }
    };

    let proton: &str = proton::find_highest_version(&proton_versions).unwrap().path.to_str().expect("Failed to get Proton");
    println!("Proton bin: {}", proton);
    proton::launch_exe("7th Heaven.exe", proton);
}
