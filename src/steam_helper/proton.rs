use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Runner {
    pub name: String,
    pub pretty_name: String,
    pub path: PathBuf,
}
use anyhow::{bail, Context, Result};

pub fn find_all_versions(steam_dir: steamlocate::SteamDir) -> Result<Vec<Runner>> {
    // TODO: custom runner support via compatibilitytools.d
    let mut proton_versions: Vec<Runner> = Vec::new();
    for library in (steam_dir.libraries()?).flatten() {
        for app in library.apps().flatten() {
            let app_name = app.name.as_ref().context("App name missing.")?;
            if app_name.contains("Proton") {
                let app_path = library.resolve_app_dir(&app).join("proton");
                if app_path.is_file() {
                    let name = app_name
                        .to_lowercase()
                        .split(".")
                        .next()
                        .context("No . found in name")?
                        .replace(" ", "_");

                    proton_versions.push(Runner {
                        name,
                        pretty_name: app_name.to_string(),
                        path: app_path,
                    });
                } else {
                    log::info!("Does not contain proton bin: {app_path:?}");
                }
            }
        }
    }

    if proton_versions.is_empty() {
        bail!("No Proton versions found")
    }
    Ok(proton_versions)
}

pub fn find_highest_version(versions: &[Runner]) -> Option<&Runner> {
    versions.iter().max_by_key(|proton| {
        let pretty_name = &proton.pretty_name;
        let version_parts: Vec<&str> = pretty_name.split_whitespace().collect();
        if version_parts.len() >= 2 && version_parts[0] == "Proton" {
            let version_str = version_parts[1]
                .split('-')
                .next()
                .unwrap_or(version_parts[1]);
            match version_str.parse::<f64>() {
                Ok(n) => ((n * 1000.0) as i64, 0), // Numeric version gets priority
                Err(_) => {
                    // Non-numeric versions like "Experimental" get lower priority
                    if version_str.to_lowercase().contains("experimental") {
                        (0, 2) // Treat "Experimental" as Proton 2.0
                    } else if version_str.to_lowercase().contains("hotfix") {
                        (0, 1) // Treat "Hotfix" as Proton 1.0
                    } else {
                        (0, 0) // Lowest priority for other non-numeric versions
                    }
                }
            }
        } else {
            (0, 0) // Default for unparseable names
        }
    })
}
