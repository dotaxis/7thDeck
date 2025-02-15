use seventh_deck::{logging, steam_helper::{self, game::SteamGame}};
use std::{
    collections::HashMap, env, error::Error, fmt::Write, fs::File, path::PathBuf, time::Duration
};
use rfd::FileDialog;
use sysinfo::System;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use console::Style;
use dialoguer::theme::ColorfulTheme;

pub static VERSION: &str = "2.5.0";
static FF7_APPID: u32 = 39140;

fn main() {
    logging::init();
    let mut config = HashMap::new();
    let current_bin = env::current_exe().expect("Failed to get binary path");
    let current_dir = current_bin.parent().expect("Failed to get binary directory");
    let toml_path = current_dir.join("7thDeck.toml");

    draw_header();

    let steam_dir = steam_helper::get_library().expect("Couldn't get Steam directory!");
    config.insert("steam_dir", steam_dir.path().display().to_string());

    let cache_dir = home::home_dir().expect("Couldn't find $HOME?").join(".cache");
    let exe_path = download_latest("tsunamods-codes/7th-Heaven", cache_dir).expect("Failed to download 7th Heaven!");

    let game = with_spinner("Finding FF7...", "Done!", || steam_helper::game::get_game(FF7_APPID, steam_dir.clone()).unwrap());

    if let Some(runner) = &game.runner {
        log::info!("Runner set for {}: {}", game.name, runner.pretty_name);
        config.insert("runner", runner.name.clone());
    }

    let toml_string = toml::to_string(&config).expect("Couldn't serialize to TOML!");
    std::fs::write(toml_path, toml_string).unwrap();

    with_spinner("Killing Steam...", "Done!", steam_helper::kill_steam);
    with_spinner("Setting Proton version...", "Done!", || steam_helper::game::set_runner(&game, "proton_9").expect("Failed to set runner")); // TODO: Expand this to allow Proton version selection
    with_spinner("Wiping prefix...", "Done!", || steam_helper::game::wipe_prefix(&game).expect("Failed to wipe prefix"));
    with_spinner("Setting Launch Options...", "Done!", || steam_helper::game::set_launch_options(&game).expect("Failed to set launch options"));
    steam_helper::game::launch_game(&game).expect("Failed to launch FF7?");
    with_spinner("Rebuilding prefix...", "Done!", || kill("FF7_Launcher"));

    let install_path = get_install_path();
    with_spinner("Installing 7th Heaven...", "Done!", || install_7th(game, exe_path, install_path, "7thHeaven.log"));
}

fn draw_header() {
    let title = format!("Welcome to 7thDeck {}", VERSION);
    let description = [
        "This script will:",
        "1. Apply patches to FF7's proton prefix to accommodate 7th Heaven",
        "2. Install 7th Heaven to a folder of your choosing",
        "3. Add 7th Heaven to Steam using a custom launcher script",
        "4. Add a custom controller config for Steam Deck, to allow mouse",
        "   control with trackpad without holding down the STEAM button",
    ];
    let footer = "For support, please open an issue on GitHub,or ask in the #ff7-linux channel of the Tsunamods Discord";

    // Pad description
    let description: Vec<String> = description
        .iter()
        .map(|line| format!("    {line}    "))
        .collect();

    // Define styles
    let border_style = Style::new().cyan(); // Cyan borders
    let title_style = Style::new().bold().cyan(); // Bold cyan title
    let text_style = Style::new().white(); // White text
    let footer_style = Style::new().dim().white(); // Dim white footer

    // Calculate the maximum line width in the description
    let max_description_width = description
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    // Calculate the banner width based on the longest description line
    let banner_width = max_description_width + 4; // 2 spaces padding + 2 border characters

    // Define border characters
    let top_border = format!("┏{}┓", "━".repeat(banner_width - 2));
    let bottom_border = format!("┗{}┛", "━".repeat(banner_width - 2));
    let middle_border = format!("┣{}┫", "━".repeat(banner_width - 2));
    let border_char = "┃";

    // Print the top border
    println!("{}", border_style.apply_to(top_border));

    // Print the title
    println!(
        "{} {:^max_description_width$} {}",
        border_style.apply_to(border_char),
        title_style.apply_to(title),
        border_style.apply_to(border_char)
    );

    // Print the middle border
    println!("{}", border_style.apply_to(&middle_border));

    // Print the description
    for line in description.iter() {
        println!(
            "{} {:<max_description_width$} {}",
            border_style.apply_to(border_char),
            text_style.apply_to(line),
            border_style.apply_to(border_char)
        );
    }

    // Print the bottom border
    println!("{}", border_style.apply_to(middle_border));

    // Wrap the footer to match the width of the longest description line
    let wrapped_footer = textwrap::fill(footer, max_description_width); // Wrap the footer text

    // Print the wrapped footer
    for line in wrapped_footer.lines() {
        println!(
            "{} {:<max_description_width$} {}",
            border_style.apply_to(border_char),
            footer_style.apply_to(line),
            border_style.apply_to(border_char)
        );
    }

    // Print the bottom border
    println!("{}", border_style.apply_to(bottom_border));

    let choices = &["Yes", "No"];
    let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .default(0) // Default to "Yes"
        .items(choices)
        .interact()
        .unwrap();

    if selection == 1 { // No
        println!("Understood. Exiting.");
        std::process::exit(0);
    }
}

fn kill(pattern: &str){
    log::info!("Waiting for prefix to rebuild.");
    'kill: loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if process.name().contains(pattern) {
                log::info!("Found '{}' with PID: {}", pattern, pid);

                if process.kill() {
                    log::info!("Killed {} successfully.", pattern);
                    break 'kill;
                } else {
                    println!("Failed to kill {}!\nPlease exit {} manually and press Enter to continue.", pattern, pattern);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
    }
    log::info!("We made it out of the kill loop!");
}

fn install_7th(game: SteamGame, exe_path: PathBuf, install_path: PathBuf, log_file: &str) {
    let args: Vec<String> = vec![
        "/VERYSILENT".to_string(),
        format!("/DIR=Z:{}", install_path.to_string_lossy().replace('/', "\\")),
        format!("/LOG={}", log_file)
    ];

    match steam_helper::game::launch_exe_in_prefix(exe_path, &game, Some(args)) {
        Ok(_) => log::info!("Ran 7th Heaven installer"),
        Err(e) => panic!("Couldn't run 7th Heaven installer: {}", e)
    }

    let current_bin = env::current_exe().expect("Failed to get binary path");
    let current_dir = current_bin.parent().expect("Failed to get binary directory");
    let toml_path = current_dir.join("7thDeck.toml");
    std::fs::copy(toml_path, install_path.join("7thDeck.toml")).expect("Failed to copy TOML to install_path");

    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let launcher_path = format!("target/{}/launcher", profile);
    std::fs::copy(launcher_path, install_path.join("Launch 7th Heaven")).expect("Failed to copy launcher to install_path");
}

fn get_install_path() -> PathBuf {
    let term = console::Term::stdout();
    println!("{} Select a destination for 7th Heaven.", console::style("+").yellow());

    loop {
        let install_path = FileDialog::new()
            .set_title("Select Destination")
            .pick_folder();

        if let Some(path) = install_path {
            let choices = &["Yes", "No"];
            let confirm = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Do you want to install 7th Heaven to '{}'?",
                console::style(path.display()).bold().underlined())) 
            .default(0) // Default to "Yes"
            .items(choices)
            .interact()
            .unwrap();

            match confirm {
                0 => {
                    term.clear_last_lines(2).unwrap();
                    println!("{} Installing to '{}'", console::style("!").yellow(),
                        console::style(path.display()).bold().underlined().white());
                    return path;
                },
                _ => {
                    term.clear_last_lines(1).unwrap();
                    continue
                }
            }
        }
    }
}

fn download_latest(repo: &str, destination: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let release_url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let response: serde_json::Value = client
        .get(&release_url)
        .header("User-Agent", "rust-client")
        .send()?
        .json()?;

    let assets = response["assets"].as_array().ok_or("No assets found")?;
    let exe_asset = assets
        .iter()
        .find(|a| a["name"].as_str().unwrap_or("").ends_with(".exe"))
        .ok_or("No .exe asset found")?;

    let download_url = exe_asset["browser_download_url"]
        .as_str()
        .ok_or("No download URL")?;

    let size = exe_asset["size"]
        .as_u64()
        .unwrap_or(0);

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", exe_asset["name"]));

    std::fs::create_dir_all(&destination)?;
    let file_name = exe_asset["name"].as_str().ok_or("Invalid file name")?;
    let file_path = destination.join(file_name);

    let mut response = client.get(download_url).send()?;
    let mut file = File::create(&file_path)?;
    let mut writer = pb.wrap_write(&mut file);
    let downloaded = response.copy_to(&mut writer)?;
    pb.set_position(downloaded);
    pb.finish_and_clear();
    pb.println(format!("{} Download complete", console::style("✔").green()));

    Ok(file_path)
}

fn with_spinner<F, T>(message: &str, success_message: &str, func: F) -> T where
    F: FnOnce() -> T,
{
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));

    let result = func(); // run the function and store result
    pb.finish_and_clear();
    println!("{} {} {}", console::style("✔").green(), message, success_message);
    result
}
