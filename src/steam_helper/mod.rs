use std::io;
use sysinfo::System;

pub mod game;
pub mod proton;

pub fn kill_steam() {
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut steam_running = false;

        for (pid, process) in sys.processes() {
            if process.name() == "steam" {
                steam_running = true;
                log::info!("Found 'steam' with PID: {}", pid);
                if process.kill() {
                    log::info!("Killed Steam successfully.");
                    return;
                } else {
                    // todo: use dialoguer -- or should we leave this?
                    log::error!("Failed to kill Steam! Please exit Steam and press A or Enter to continue.");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
        if !steam_running {
            log::warn!("I guess Steam isn't running. Continuing.");
            break;
        }
    }
}
