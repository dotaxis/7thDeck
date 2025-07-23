use std::{
    error::Error,
    fs::{self, metadata},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use regex::Regex;
use glob::glob;
use super::proton::{self, Runner};

#[derive(Debug)]
pub struct SteamGame {
    pub app_id: u32,
    pub name: String,
    pub path: PathBuf,
    pub prefix: PathBuf,
    pub client_path: PathBuf,
    pub runner: Option<Runner>,
}

pub fn get_game(app_id: u32, steam_dir: steamlocate::SteamDir) -> Result<SteamGame, Box<dyn Error>> {
    let steam_dir_pathbuf = PathBuf::from(steam_dir.path());
    log::info!("Located Steam installation: {}", steam_dir_pathbuf.display());
    for library in steam_dir.libraries()? {
        let library = match library {
            Ok(library) => library,
            Err(e) => {
                log::warn!("Couldn't access library: {}", e);
                continue
            }
        };
        for app_result in library.apps() {
            let app = app_result?;
            if app.app_id == app_id {
                let name = app.name.ok_or("No app name?")?;
                let path = library.path().join(format!("steamapps/common/{}", name));
                let prefix = library.path().join(format!("steamapps/compatdata/{}/pfx", app_id));
                let runner = steam_dir.compat_tool_mapping()
                    .unwrap_or_default()
                    .get(&app_id)
                    .and_then(|tool| {
                        let tool_name = tool.name.clone()?;
                        let proton_versions = proton::find_all_versions(steam_dir.clone()).ok()?;
                        proton_versions.into_iter()
                            .find(|runner| runner.name == tool_name)
                    });

                let steam_game = SteamGame {
                    app_id,
                    name,
                    path,
                    prefix,
                    client_path: steam_dir_pathbuf,
                    runner,
                };

                return Ok(steam_game);
            }
        }
    }

    Err(format!("Couldn't find app_id {}!", app_id).into())
}

pub fn launch_exe_in_prefix(exe_to_launch: PathBuf, game: &SteamGame, args: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let proton = game.runner.clone().unwrap().path;
    log::info!("Proton bin: {:?}", proton);

    let mut command = Command::new(proton);
    command
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &game.client_path)
        .env("STEAM_COMPAT_DATA_PATH", game.prefix.parent().unwrap())
        .env("WINEDLLOVERRIDES", "dinput.dll=n,b")
        .stdout(Stdio::null()).stderr(Stdio::null()) // &> /dev/null
        .arg("waitforexitandrun")
        .arg(&exe_to_launch);
    let args = args.unwrap_or_default();
    for arg in args {
        log::info!("launch_exe_in_prefix arg: {}", arg);
        command.arg(arg);
    }
    command.spawn()?.wait()?;

    log::info!("Launched {}", exe_to_launch.file_name().unwrap().to_string_lossy());
    Ok(())
}

pub fn wipe_prefix(game: &SteamGame) -> Result<(), Box<dyn std::error::Error>> {
    let prefix_dir = match metadata(Path::new(&game.prefix)) {
        Ok(meta) if meta.is_dir() => Path::new(&game.prefix),
        _ => {
            log::info!("{} doesn't exist. Continuing.", game.prefix.display());
            return Ok(())
        }
    };

    // Better safe than sorry
    let pattern = format!("compatdata/{}/pfx", &game.app_id);
    if !prefix_dir.to_string_lossy().contains(&pattern) {
        panic!("{} does not contain {}", prefix_dir.display(), pattern);
    }

    log::info!("Deleting path: {}", prefix_dir.display());
    std::fs::remove_dir_all(prefix_dir)?;
    log::info!("Wiped prefix for app_id: {}", &game.app_id);
    Ok(())
 }

pub fn set_launch_options(game: &SteamGame) -> Result<(), Box<dyn std::error::Error>> {
    // Set launch options for Steam injection
    let re = Regex::new(&format!(r#""{}"\s*\{{"#, &game.app_id))?;
    let replacement = format!(
        r#""{}"
					{{
						"LaunchOptions"		"echo \"%command%\" | sed 's/waitforexitandrun/run/g' | env WINEDLLOVERRIDES=\"dinput=n,b\" sh""#,
        &game.app_id
    );

    for entry in glob(&format!("{}/userdata/*/config/localconfig.vdf", &game.client_path.display()))? {
        let path = entry?;
        log::info!("localconfig.vdf found at {:?}", path);
        let content = fs::read_to_string(&path)?;
        fs::write(&path, re.replace(&content, &replacement).as_bytes())
            .unwrap_or_else(|e| panic!("Couldn't write to {:?}: {}", path, e));
    }
    log::info!("Successfully set launch options for {}", game.name);
    Ok(())
}

pub fn set_runner(game: &SteamGame, runner: &str) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r#""CompatToolMapping"\s*\{"#)?;
    let replacement = format!(
        r#""CompatToolMapping"
				{{
					"{}"
					{{
						"name"		"{}"
						"config"		""
						"priority"		"250"
					}}"#,
        &game.app_id, runner
    );
    let path = &game.client_path.join("config/config.vdf");
    let content = fs::read_to_string(path)?;
    fs::write(path, re.replace(&content, replacement).as_bytes())
        .unwrap_or_else(|e| panic!("Couldn't write to {:?}: {}", path, e));
    log::info!("Succcessfully set runner for {} to {}", &game.app_id, runner);
    Ok(())
}

pub fn launch_game(game: &SteamGame) -> Result<(), Box<dyn Error>> {
    let steam_command = format!("steam://rungameid/{:?}", &game.app_id);
    log::info!("Running command: steam {}", &steam_command);
    // nohup steam steam://rungameid/39140 &> /dev/null
    Command::new("steam")
        .arg(steam_command)
        .stdout(Stdio::null()).stderr(Stdio::null()) // &> /dev/null
        .spawn()?;

    log::info!("Launched {}", game.name);
    Ok(())
}
