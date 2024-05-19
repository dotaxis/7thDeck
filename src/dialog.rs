use std::process::Command;

pub fn prompt_ok(dialog_command: &str, message: String) {
    let mut command = Command::new(dialog_command);
    let args: Vec<String>;
    match dialog_command {
        "kdialog" => args = vec!["--msgbox".to_string(), message],
        "zenity" => args = vec!["--info".to_string(), format!("--text={}", message)],
        "dialog" => args = vec!["--msgbox".to_string(), message, 10.to_string(), 60.to_string()],
        _ => panic!("No dialog command??")
    }
    for arg in args { command.arg(arg); }
    command.spawn().expect("???").wait().expect("???");
}

pub fn which_dialog() -> String {
    if let Ok(output) = Command::new("which").arg("kdialog").output() {
        if output.status.success() {
            return "kdialog".to_string();
        }
    }

    if let Ok(output) = Command::new("which").arg("zenity").output() {
        if output.status.success() {
            return "zenity".to_string();
        }
    }

    "dialog".to_string()
}
