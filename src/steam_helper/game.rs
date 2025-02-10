use keyvalues_parser::{Vdf, Value};
use std::{
    error::Error,
    fs::{self, metadata},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use regex::Regex;
use glob::glob;

#[derive(Debug)]
pub struct SteamGame {
    pub app_id: u32,
    pub name: String,
    pub path: PathBuf,
    pub prefix: PathBuf,
    pub client_path: PathBuf
}

pub fn get_game(app_id: u32, steam_dir: steamlocate::SteamDir) -> Result<SteamGame, Box<dyn Error>> {
    let steam_dir_pathbuf = PathBuf::from(steam_dir.path());
    log::info!("Located Steam installation: {}", steam_dir_pathbuf.display());
    for library in steam_dir.libraries()? {
        let library = library?;
        for app_result in library.apps() {
            let app = app_result?;
            if app.app_id == app_id {
                let name = app.name.ok_or("No app name?")?;
                let path = library.path().join(format!("steamapps/common/{}", name));
                let prefix = library.path().join(format!("steamapps/compatdata/{}/pfx", app_id));

                let steam_game = SteamGame {
                    app_id,
                    name,
                    path,
                    prefix,
                    client_path: steam_dir_pathbuf
                };

                return Ok(steam_game);
            }
        }
    }

    Err(format!("Couldn't find app_id {}!", app_id).into())
}

pub fn launch_exe_in_prefix(exe_to_launch: PathBuf, game: &SteamGame, proton_path: &str, args: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new(proton_path);
    command
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &game.client_path)
        .env("STEAM_COMPAT_DATA_PATH", game.prefix.as_path())
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

pub fn get_runner(game: &SteamGame) -> Result<String, Box<dyn Error>> {
    let path = game.client_path.join("config/config.vdf");
    let vdf_data = fs::read_to_string(path).expect("Failed to read config.vdf");

    if let Ok(vdf) = Vdf::parse(&vdf_data) {
        let name_str = {
            if let Value::Obj(root_obj) = &vdf.value {
                root_obj.get("Software")
                    .and_then(|software| software.as_slice().first())
                    .and_then(|valve_obj|
                if let Value::Obj(valve_obj) = valve_obj { Some(valve_obj) } else { None })
                    .and_then(|valve_obj| valve_obj.get("Valve"))
                    .and_then(|valve| valve.as_slice().first())
                    .and_then(|steam_obj|
                if let Value::Obj(steam_obj) = steam_obj { Some(steam_obj) } else { None })
                    .and_then(|steam_obj| steam_obj.get("Steam"))
                    .and_then(|steam| steam.as_slice().first())
                    .and_then(|compat_obj|
                if let Value::Obj(compat_obj) = compat_obj { Some(compat_obj) } else { None })
                    .and_then(|compat_obj| compat_obj.get("CompatToolMapping"))
                    .and_then(|compat| compat.as_slice().first())
                    .and_then(|game_obj|
                if let Value::Obj(game_obj) = game_obj { Some(game_obj) } else { None })
                    .and_then(|game_obj| game_obj.get("39140"))
                    .and_then(|game| game.as_slice().first())
                    .and_then(|props_obj|
                if let Value::Obj(props_obj) = props_obj { Some(props_obj) } else { None })
                    .and_then(|props_obj| props_obj.get("name"))
                    .and_then(|name| name.as_slice().first())
                    .and_then(|name_str|
                if let Value::Str(name_str) = name_str { Some(name_str) } else { None })
            } else {
                None
            }
        };

        match name_str {
            Some(name) => {
                log::info!("Found runner for app_id {}: {}", game.app_id, name.to_string());
                return Ok(name.to_string());
            },
            _ => panic!("Couldn't find runner in config.vdf"),
        }
    }

    panic!("Failed to parse config.vdf");
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
