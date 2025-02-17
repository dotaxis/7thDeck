use std::path::PathBuf;

pub const LOGO_PNG: &[u8] = include_bytes!("../resources/logo.png");
pub const TIMEOUT_EXE: &[u8] = include_bytes!("../resources/timeout.exe");

pub const CONTROLLER_PROFILE: &str = include_str!("../resources/controller_neptune_gamepad+mouse+click.vdf");
pub const DEFAULT_XML: &str = include_str!("../resources/Default.xml");
pub const MOD_XML: &str = include_str!("../resources/mod.xml");
pub const SETTINGS_XML: &str = include_str!("../resources/settings.xml");

#[derive(Debug)]
pub struct FileAsStr {
    pub name: String,
    pub destination: PathBuf,
    pub contents: String,
}

#[derive(Debug)]
pub struct FileAsBytes {
    pub name: String,
    pub destination: PathBuf,
    pub contents: Vec<u8>,
}


pub fn as_bytes(name: String, destination: PathBuf, contents: &[u8]) -> FileAsBytes {
    FileAsBytes { name, destination, contents: contents.to_vec() }
}


pub fn as_str(name: String, destination: PathBuf, contents: &str) -> FileAsStr {
    FileAsStr { name, destination, contents: contents.to_string() }
}
