use std::process::Command;

fn main() {
    println!("Building launcher!");
    Command::new("cargo")
        .args(["build", "--bin", "launcher"])
        .status()
        .expect("Failed to build launcher");
}
