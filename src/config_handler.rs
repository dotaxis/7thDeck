use anyhow::{Context, Result};
use std::{collections::HashMap, env};

static CONFIG_NAME: &str = "7thDeck.toml";

pub fn write(config: HashMap<&str, String>) -> Result<()> {
    let current_bin = env::current_exe()?;
    let current_dir = current_bin
        .parent()
        .expect("Failed to get current directory");
    let toml_path = current_dir.join(CONFIG_NAME);

    let toml_string = toml::to_string(&config)?;
    std::fs::write(toml_path, toml_string)?;

    Ok(())
}

pub fn read_value(key: &str) -> Result<String> {
    let current_bin = env::current_exe().context("Failed to get binary path")?;
    let current_dir = current_bin
        .parent()
        .context("Failed to get binary directory")?;
    let toml_path = current_dir.join(CONFIG_NAME);

    let toml_string = std::fs::read_to_string(toml_path).context("Couldn't read TOML")?;
    let toml_value: toml::Value =
        toml::from_str(&toml_string).context("Couldn't deserialize TOML")?;

    let value = toml_value
        .get(key)
        .with_context(|| format!("Couldn't find {key} key in {CONFIG_NAME}"))?
        .as_str()
        .with_context(|| format!("{key} value is not a string"))?
        .to_string();

    Ok(value.to_string())
}
