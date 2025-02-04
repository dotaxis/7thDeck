use std::{
    error::Error,
    fs::metadata,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    fs
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

pub fn get_game(app_id: u32) -> Result<SteamGame, Box<dyn Error>> {
    let steam_dir = steamlocate::SteamDir::locate()?;
    let steam_dir_pathbuf = PathBuf::from(steam_dir.path());
    println!("Located Steam installation: {}", steam_dir_pathbuf.display());

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

pub fn launch_exe_in_prefix(exe_to_launch: PathBuf, game: SteamGame, proton_path: &str, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new(proton_path);
    command
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", game.client_path)
        .env("STEAM_COMPAT_DATA_PATH", game.prefix.as_path())
        .env("WINEDLLOVERRIDES", "dinput.dll=n,b")
        .stdout(Stdio::null()).stderr(Stdio::null()) // &> /dev/null
        .arg("waitforexitandrun")
        .arg(&exe_to_launch);
    for arg in args {
        println!("launch_exe_in_prefix arg: {}", arg);
        command.arg(arg);
    }
    command.spawn()?.wait()?;

    Ok(println!("Launched {}", exe_to_launch.file_name().unwrap().to_string_lossy()))
}

pub fn wipe_prefix(game: &SteamGame) {
    println!("Hello my name is WIPE_PREFIX");
    let prefix_dir = match metadata(Path::new(&game.prefix)).unwrap().is_dir() {
        true => {
            Path::new(&game.prefix)
        },
        false => panic!("{} is not a directory!", game.prefix.to_string_lossy())
    };

    // Better safe than sorry
    let pattern = format!("compatdata/{}/pfx", &game.app_id);
    if !prefix_dir.to_string_lossy().contains(&pattern) {
        panic!("{} does not contain {}", prefix_dir.display(), pattern);
    }

    println!("Deleting path: {}", prefix_dir.display());
    std::fs::remove_dir_all(prefix_dir).expect("Failed to delete path!");
    println!("Wiped prefix for app_id: {}", &game.app_id);
 }

pub fn set_launch_options(game: &SteamGame) -> Result<(), Box<dyn std::error::Error>> {
    // Set launch options for Steam injection

    let re = Regex::new(&format!(r#"("{}")\s*\{{)"#, &game.app_id))?;
    let replacement = r#"$1
    "LaunchOptions"		"echo \"%command%\" | sed 's/waitforexitandrun/run/g' | env WINEDLLOVERRIDES=\"dinput=n,b\" sh"
    "#;

    for entry in glob(&format!("{}/userdata/*/config/localconfig.vdf", &game.client_path.display()))? {
        let path = entry?;
        let content = fs::read_to_string(&path)?;
        fs::write(&path, re.replace(&content, replacement).as_bytes())?;
    }
    Ok(())
}

pub fn launch_game(game: &SteamGame) -> Result<(), Box<dyn Error>> {
    let steam_command = format!("steam://rungameid/{:?}", &game.app_id);
    println!("Running command: steam {}", &steam_command);
    // nohup steam steam://rungameid/39140 &> /dev/null
    Command::new("steam")
        .arg(steam_command)
        .stdout(Stdio::null()) // &> /dev/null
        .stderr(Stdio::null()) // &> /dev/null
        .spawn()?;

    Ok(println!("Launched {}", game.name))
}
