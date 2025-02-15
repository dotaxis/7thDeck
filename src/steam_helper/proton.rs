use std::{error::Error, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Runner {
    pub name: String,
    pub pretty_name: String,
    pub path: PathBuf,
}

pub fn find_all_versions(steam_dir: steamlocate::SteamDir) -> Result<Vec<Runner>, Box<dyn Error>> {
    // TODO: custom runner support via compatibilitytools.d
    let mut proton_versions: Vec<Runner> = Vec::new();
    for library in (steam_dir.libraries()?).flatten() {
        for app in library.apps().flatten() {
            let app_name = app.name.as_ref().unwrap();
            if app_name.contains("Proton") {
                let app_path = library.resolve_app_dir(&app).join("proton");
                if app_path.is_file() {
                    let name = app_name
                        .to_lowercase()
                        .split(".")
                        .next()
                        .unwrap()
                        .replace(" ", "_");

                    proton_versions.push(
                            Runner {
                                name,
                                pretty_name: app_name.to_string(),
                                path: app_path,
                            }
                        );
                } else {
                    log::info!("Does not contain proton bin: {:?}", app_path);
                }
            }
        }
    }

    proton_versions
        .is_empty()
        .then(|| Err("No Proton versions found".into()))
        .unwrap_or(Ok(proton_versions))
}

pub fn find_highest_version(versions: &[Runner]) -> Option<&Runner> {
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
