use std::{error::Error, path::PathBuf};

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
