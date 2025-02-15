use std::{collections::HashMap, env, error::Error};

static CONFIG_NAME: &str = "7thDeck.toml";

pub fn write(config: HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
    let current_bin = env::current_exe()?;
    let current_dir = current_bin.parent().expect("Failed to get current directory");
    let toml_path = current_dir.join(CONFIG_NAME);

    let toml_string = toml::to_string(&config)?;
    std::fs::write(toml_path, toml_string)?;

    Ok(())
}

pub fn read_value(key: &str) -> String {
    let current_bin = env::current_exe().expect("Failed to get binary path");
    let current_dir = current_bin.parent().expect("Failed to get binary directory");
    let toml_path = current_dir.join(CONFIG_NAME);

    let toml_string = std::fs::read_to_string(toml_path).expect("Couldn't read TOML");
    let toml_value: toml::Value = toml::from_str(&toml_string).expect("Couldn't deserialize TOML");
    
    toml_value.get(key)
        .unwrap_or_else(|| panic!("Couldn't find {} key in {}", key, CONFIG_NAME))
        .as_str()
        .unwrap_or_else(|| panic!("{} value is not a string", key))
        .to_string()
}
