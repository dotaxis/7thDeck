use std::{env, error::Error, path::{PathBuf, Path}, process::Command};

#[derive(Debug)]
pub struct ProtonVersion {
    pub name: String,
    pub path: PathBuf
}

pub fn find_all_versions() -> Result<Vec<ProtonVersion>, Box<dyn Error>> {
    let steam_dir = steamlocate::SteamDir::locate()?;
    let mut proton_versions: Vec<ProtonVersion> = Vec::new();
    for library in (steam_dir.libraries()?).flatten() {
        for app in library.apps().flatten() {
            let app_name = app.name.as_ref().unwrap();
            if app_name.contains("Proton") {
                let app_path = library.resolve_app_dir(&app).join("proton");
                if app_path.is_file() {
                    proton_versions
                        .push(
                            ProtonVersion {
                                name: app_name.to_string(),
                                path: app_path
                            }
                        );
                } else {
                    eprintln!("{:?}", app_path);
                }
            }
        }
    }
    if proton_versions.is_empty() {
        Err("No Proton versions found".into())
    } else {
        Ok(proton_versions)
    }
}

pub fn find_highest_version(versions: &[ProtonVersion]) -> Option<&ProtonVersion> {
    versions.iter().max_by_key(|proton| {
        let version_parts: Vec<&str> = proton.name.split_whitespace().collect();
        if let Some(version) = version_parts.get(1) {
            match version.parse::<f64>() {
                Ok(n) => (n as i64, 0), // Numeric version
                Err(_) => {
                    if version.contains("Experimental") {
                        (2, 0) // Treat "Experimental" as Proton 2.0
                    } else if version.contains("Hotfix") {
                        (1, 0) // Treat "Hotfix" as Proton 1.0
                    } else {
                        (0, 0) // Non-numeric versions or special cases not handled above
                    }
                }
            }
        } else {
            (0, 0) // Default for non-parsable or missing version numbers
        }
    })
}

pub fn find_prefix(app_id: u32) -> Result<PathBuf, Box<dyn Error>> {
    let steam_dir = steamlocate::SteamDir::locate()?;
    println!("Located Steam installation: {}", steam_dir.path().display());

    for library in steam_dir.libraries()? {
        let library = library?;
        for app in library.apps() {
            if app?.app_id == app_id {
                let prefix_path = library.path().join(format!("steamapps/compatdata/{}/pfx", app_id));
                println!("Prefix path: {:?}", prefix_path);
                return Ok(prefix_path);
            }
        }
    }
    Err("Couldn't find proton prefix!".into())
}

pub fn launch_exe_in_prefix(app_id: u32, exe_path: &str, proton_path: &str, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let prefix = find_prefix(app_id);
    let mut command = Command::new(proton_path);

    command
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", steamlocate::SteamDir::locate()?.path())
        .env("STEAM_COMPAT_DATA_PATH", prefix?.as_path())
        .env("WINEDLLOVERRIDES", "dinput.dll=n,b")
        .arg("run")
        .arg(exe_path);

    for arg in args { command.arg(arg); }

    command.spawn()?.wait()?;

    Ok(println!("Launched {exe_path}"))
}
