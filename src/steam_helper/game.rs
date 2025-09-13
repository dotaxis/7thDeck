use super::proton::{self, Runner};
use anyhow::{bail, Context, Result};
use glob::glob;
use regex::Regex;
use std::{
    fs::{self, metadata},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct SteamGame {
    pub app_id: u32,
    pub name: String,
    pub path: PathBuf,
    pub prefix: PathBuf,
    pub client_path: PathBuf,
    pub runner: Option<Runner>,
}

pub fn get_game(app_id: u32, steam_dir: steamlocate::SteamDir) -> Result<SteamGame> {
    let steam_dir_pathbuf = PathBuf::from(steam_dir.path());
    log::info!(
        "Located Steam installation: {}",
        steam_dir_pathbuf.display()
    );
    let (app, library) = steam_dir.find_app(app_id)?.unwrap();
    let path = library.resolve_app_dir(&app);
    let name = app.name.context("No app name?")?.to_string();
    let prefix = library
        .path()
        .join(format!("steamapps/compatdata/{app_id}/pfx"));
    let runner = steam_dir
        .compat_tool_mapping()
        .with_context(|| format!("Couldn't get runner for {app_id}"))?
        .get(&app_id)
        .and_then(|tool| {
            let tool_name = tool.name.clone()?;
            let proton_versions = proton::find_all_versions(steam_dir.clone()).ok()?;
            proton_versions
                .into_iter()
                .find(|runner| runner.name == tool_name)
        })
        .or_else(|| {
            log::info!(
                "No runner configured for app {}, setting highest version",
                app_id
            );
            let proton_versions = proton::find_all_versions(steam_dir.clone()).ok()?;
            let highest_runner = proton::find_highest_version(&proton_versions)?;

            let temp_game = SteamGame {
                app_id,
                name: name.clone(),
                path: path.clone(),
                prefix: prefix.clone(),
                client_path: steam_dir_pathbuf.clone(),
                runner: None,
            };

            if set_runner(&temp_game, &highest_runner.name).is_ok() {
                log::info!(
                    "Set runner to '{}' for app {}",
                    highest_runner.pretty_name,
                    app_id
                );
                Some(highest_runner.clone())
            } else {
                log::warn!("Failed to set runner for app {}", app_id);
                None
            }
        });

    let steam_game = SteamGame {
        app_id,
        name,
        path,
        prefix,
        client_path: steam_dir_pathbuf,
        runner,
    };

    Ok(steam_game)
}

pub fn launch_exe_in_prefix(
    exe_to_launch: PathBuf,
    game: &SteamGame,
    args: Option<Vec<String>>,
    use_runtime: Option<SteamGame>,
) -> Result<()> {
    let proton = game
        .runner
        .clone()
        .with_context(|| format!("Game has no runner? {game:?}"))?
        .path;
    log::info!("Proton bin: {proton:?}");

    let mut command: Command;
    if let Some(runtime) = use_runtime {
        let runtime_path = runtime.path.join("run");
        log::info!("{} path: {runtime_path:?}", runtime.name);
        command = Command::new(runtime_path);
        command
            .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &game.client_path)
            .env(
                "STEAM_COMPAT_DATA_PATH",
                game.prefix
                    .parent()
                    .context("Couldn't get parent of prefix directory")?,
            )
            .env("WINEDLLOVERRIDES", "dinput.dll=n,b")
            .stdout(Stdio::null())
            .stderr(Stdio::null()) // TODO: log this properly
            .arg("--")
            .arg(proton)
            .arg("waitforexitandrun")
            .arg(&exe_to_launch);
        let args = args.unwrap_or_default();
        for arg in args {
            log::info!("launch_exe_in_prefix arg: {arg}");
            command.arg(arg);
        }
    } else {
        // no runtime
        command = Command::new(proton);
        command
            .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &game.client_path)
            .env(
                "STEAM_COMPAT_DATA_PATH",
                game.prefix
                    .parent()
                    .context("Couldn't get parent of prefix directory")?,
            )
            .env("WINEDLLOVERRIDES", "dinput.dll=n,b")
            .stdout(Stdio::null())
            .stderr(Stdio::null()) // TODO: log this properly
            .arg("waitforexitandrun")
            .arg(&exe_to_launch);
        let args = args.unwrap_or_default();
        for arg in args {
            log::info!("launch_exe_in_prefix arg: {arg}");
            command.arg(arg);
        }
    }
    command.spawn()?.wait()?;
    log::info!(
        "Launched {}",
        exe_to_launch
            .file_name()
            .context("Couldn't get file_name")?
            .to_string_lossy()
    );

    Ok(())
}

pub fn wipe_prefix(game: &SteamGame) -> Result<()> {
    let prefix_dir = match metadata(Path::new(&game.prefix)) {
        Ok(meta) if meta.is_dir() => Path::new(&game.prefix),
        _ => {
            log::info!("{} doesn't exist. Continuing.", game.prefix.display());
            return Ok(());
        }
    };

    // Better safe than sorry
    let pattern = format!("compatdata/{}/pfx", &game.app_id);
    if !prefix_dir.to_string_lossy().contains(&pattern) {
        bail!("{} does not contain {}", prefix_dir.display(), pattern);
    }

    log::info!("Deleting path: {}", prefix_dir.display());
    std::fs::remove_dir_all(prefix_dir)?;
    log::info!("Wiped prefix for app_id: {}", &game.app_id);
    Ok(())
}

pub fn set_launch_options(game: &SteamGame) -> Result<()> {
    // Set launch options for Steam injection
    let re = Regex::new(&format!(r#""{}"\s*\{{"#, &game.app_id))?;
    let replacement = format!(
        r#""{}"
					{{
						"LaunchOptions"		"echo \"%command%\" | sed 's/waitforexitandrun/run/g' | env WINEDLLOVERRIDES=\"dinput=n,b\" sh""#,
        &game.app_id
    );

    for entry in glob(&format!(
        "{}/userdata/*/config/localconfig.vdf",
        &game.client_path.display()
    ))? {
        let path = entry?;
        log::info!("localconfig.vdf found at {path:?}");
        let content = fs::read_to_string(&path)?;
        fs::write(&path, re.replace(&content, &replacement).as_bytes())
            .with_context(|| format!("Couldn't write to {path:?}"))?;
    }
    log::info!("Successfully set launch options for {}", game.name);
    Ok(())
}

pub fn set_runner(game: &SteamGame, runner: &str) -> Result<()> {
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
        .with_context(|| format!("Couldn't write to {path:?}"))?;
    log::info!(
        "Succcessfully set runner for {} to {}",
        &game.app_id,
        runner
    );
    Ok(())
}

pub fn launch_game(game: &SteamGame) -> Result<()> {
    let steam_command = format!("steam://rungameid/{:?}", &game.app_id);
    log::info!("Running command: steam {}", &steam_command);
    // nohup steam steam://rungameid/39140 &> /dev/null
    Command::new("steam")
        .arg(steam_command)
        .stdout(Stdio::null())
        .stderr(Stdio::null()) // TODO: log this properly
        .spawn()?;

    log::info!("Launched {}", game.name);
    Ok(())
}

pub fn wipe_common(game: &SteamGame) -> Result<()> {
    let common_dir = match metadata(Path::new(&game.path)) {
        Ok(meta) if meta.is_dir() => Path::new(&game.path),
        _ => {
            log::info!("{} doesn't exist. Continuing.", game.path.display());
            return Ok(());
        }
    };

    // Better safe than sorry
    let pattern = format!("common/{}", &game.name);
    if !common_dir.to_string_lossy().contains(&pattern) {
        bail!("{} does not contain {pattern}", common_dir.display());
    }

    log::info!("Deleting path: {}", common_dir.display());
    std::fs::remove_dir_all(common_dir)?;
    log::info!("Wiped prefix for app_id: {}", &game.app_id);
    Ok(())
}

pub fn validate_game(game: &SteamGame) -> Result<()> {
    let steam_command = format!("steam://validate/{:?}", &game.app_id);
    log::info!("Running command: steam {}", &steam_command);
    // nohup steam steam://validate/39140 &> /dev/null
    Command::new("steam")
        .arg(steam_command)
        .stdout(Stdio::null())
        .stderr(Stdio::null()) // TODO: log this properly
        .spawn()?;

    log::info!("Launched {}", game.name);
    Ok(())
}
