pub const LOGO_PNG: &[u8] = include_bytes!("../resources/logo.png");
pub const TIMEOUT_EXE: &[u8] = include_bytes!("../resources/timeout.exe");

pub const CONTROLLER_PROFILE: &str = include_str!("../resources/controller_neptune_gamepad+mouse+click.vdf");
pub const DEFAULT_XML: &str = include_str!("../resources/Default.xml");
pub const MOD_XML: &str = include_str!("../resources/mod.xml");
pub const SETTINGS_XML: &str = include_str!("../resources/settings.xml");

// Here is where we will store methods which handle string replacement and copying these files into the install path and FF7 prefix
