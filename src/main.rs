use seventh_deck::steamhelper;
use std::{
    error::Error, fmt::Write, fs::File, path::PathBuf
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use sysinfo::System;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use console::Style;
use dialoguer::theme::ColorfulTheme;

pub static VERSION: &str = "2.5.0";
static FF7_APPID: u32 = 39140;

fn main() {
    draw_header();
    // let game = steamhelper::game::get_game(FF7_APPID).unwrap();
    // steamhelper::kill_steam();
    // steamhelper::game::set_runner(&game, "proton_9").expect("Failed to set runner"); // TODO: Expand this to allow Proton version selection
    // steamhelper::game::wipe_prefix(&game).expect("Failed to wipe prefix");
    // steamhelper::game::set_launch_options(&game).expect("Failed to set launch options");
    // steamhelper::game::launch_game(&game).expect("Failed to launch FF7?");
    // kill("FF7_Launcher");

    let exe_name = "7th_Heaven.exe";
    // download_latest("tsunamods-codes/7th-Heaven", exe_name).expect("Failed to download 7th Heaven!");

    // let install_path = get_install_path();
    // MessageDialog::new()
    //     .set_type(MessageType::Info)
    //     .set_title("Path confirmed.")
    //     .set_text(&format!("Installing 7th Heaven to {:#?}", install_path.to_string_lossy()))
    //     .show_alert()
    //     .unwrap();

    // install_7th(exe_name, install_path, "7thHeaven.log");
}

fn draw_header() {
    let title = format!("Welcome to 7thDeck {}", VERSION);
    let description = [
        "This script will:",
        "1. Apply patches to FF7's proton prefix to accommodate 7th Heaven",
        "2. Install 7th Heaven to a folder of your choosing",
        "3. Add 7th Heaven to Steam using a custom launcher script",
        "4. Add a custom controller config for Steam Deck, to allow mouse",
        "    control with trackpad without holding down the STEAM button",
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

    if selection == 1 { // "No" is selected
        println!("Understood. Exiting.");
        std::process::exit(1);
    }
}

fn kill(pattern: &str){
    println!("Waiting for prefix to rebuild.");
    'kill: loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if process.name().contains(pattern) {
                println!("Found '{}' with PID: {}", pattern, pid);

                if process.kill() {
                    println!("Killed {} successfully.", pattern);
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
    println!("We made it out of the kill loop!");
}

fn install_7th(exe_path: &str, install_path: PathBuf, log_file: &str) {
    let proton_versions = steamhelper::proton::find_all_versions().expect("Failed to find any Proton versions!");
    let highest_proton_version = steamhelper::proton::find_highest_version(&proton_versions).unwrap();
    let proton = highest_proton_version.path.to_str().expect("Failed to get Proton").to_string();
    println!("Proton bin: {}", proton);

    let args: Vec<String> = vec![
        "/VERYSILENT".to_string(),
        format!("/DIR=Z:{}", install_path.to_string_lossy().replace('/', "\\")),
        format!("/LOG={}", log_file)
    ];

    let game = steamhelper::game::get_game(FF7_APPID).unwrap();

    match steamhelper::game::launch_exe_in_prefix(exe_path.into(), &game, &proton, Some(args)) {
        Ok(_) => println!("Ran 7th Heaven installer"),
        Err(e) => panic!("{}", e)
    }

    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let launcher_path = format!("target/{}/launcher", profile);
    std::fs::copy(launcher_path, install_path.join("launcher")).expect("Failed to copy launcher to install_path");
}

fn get_install_path() -> PathBuf {
    println!("Select an installation path for 7th Heaven.");
    loop {
        MessageDialog::new()
            .set_text("Select an installation path for 7th Heaven.")
            .set_title("Select Destination")
            .show_alert()
            .unwrap();

        let install_path = match FileDialog::new()
            .set_location("~")
            .set_title("Select Destination")
            .show_open_single_dir()
            .unwrap() {
                Some(path) => path,
                None => {
                    println!("No path selected. Retrying.");
                    continue
                }
            };

        let confirmed = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Confirm Install Location")
            .set_text(&format!("7th Heaven will be installed to:\n{:#?}\nConfirm?", install_path))
            .show_confirm()
            .unwrap();

        if !confirmed {
            println!("User did not confirm installation path. Retrying.");
            continue;
        }

        return install_path;
    }
}

fn download_latest(repo: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
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

    println!("Downloading {}", exe_asset["name"]);

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut response = client.get(download_url).send()?;
    let mut file = File::create(output_path)?;
    let mut writer = pb.wrap_write(&mut file);
    let downloaded = response.copy_to(&mut writer)?;
    pb.set_position(downloaded);
    pb.finish();

    Ok(println!("Download complete"))
}
