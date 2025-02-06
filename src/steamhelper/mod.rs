use std::{error::Error, io, path::PathBuf, process::Command, time::Duration};
use indicatif::{ProgressBar, ProgressStyle};
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
                // LOG: pb.println(format!("Found 'steam' with PID: {}", pid));
                if process.kill() {
                    // LOG: pb.println("Killed Steam successfully.");
                    return;
                } else {
                    // todo: use dialoguer
                    println!("Failed to kill Steam! Please exit Steam and press Enter to continue.");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    continue;
                }
            }
        }
        if !steam_running {
            // LOG: println!("I guess Steam isn't running. Continuing.");
            break;
        }
    }
}
