use std::process::Command;
use colored_text::Colorize;
use crate::conf::vars::{MAIN_THEME};

pub fn winget_install(program_id: &str) {
    let program_name = program_id.splitn(2, '.').nth(1).unwrap_or(program_id);

    let installing_message: String = format!(
        "Installing {}...",
        &program_name.hex(MAIN_THEME.primary)
    );
    let install_command: String = format!(
        "winget install -e --id {} --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
        &program_id
    );

    println!("{}", installing_message);

    let install_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(install_command)
        .output()
        .expect("Failed to install program via winget...");

    if !install_command_output.status.success() {
        if
            String::from_utf8_lossy(&install_command_output.stdout).contains(
                "Found an existing package"
            )
        {
            eprintln!("{}", "   This package is already installed!".hex(MAIN_THEME.warning));
        } else {
            eprintln!(
                "{}",
                format!(
                    "PowerShell returned an error:\n{}",
                    String::from_utf8_lossy(&install_command_output.stdout)
                ).hex(MAIN_THEME.error)
            );
        }
    } else {
        println!(
            "{}",
            String::from_utf8_lossy(&install_command_output.stdout).hex(MAIN_THEME.info)
        );
    }
}

pub fn winget_remove(program_id: &str) {
    let program_name = program_id.splitn(2, '.').nth(1).unwrap_or(program_id);

    let removing_message: String = format!("Removing {}...", &program_name.hex(MAIN_THEME.error));
    let remove_command: String = format!("remove -e --id {}", &program_id);

    println!("{}\n", removing_message);

    let remove_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(remove_command)
        .output()
        .expect("Failed to remove program via winget...");

    if !remove_command_output.status.success() {
        eprintln!(
            "{}",
            format!(
                "PowerShell returned an error:\n{}",
                String::from_utf8_lossy(&remove_command_output.stdout)
            ).hex(MAIN_THEME.error)
        );
    } else {
        println!("{}", String::from_utf8_lossy(&remove_command_output.stdout).hex(MAIN_THEME.info));
    }
}