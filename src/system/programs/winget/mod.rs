use crate::conf::{enums::DelimiterType, enums::MessageType, vars::MAIN_THEME};
use crate::utils::message;
use colored_text::Colorize;
use std::process::Command;

pub fn winget_install(program_id: &str) {
    let program_name = program_id.splitn(2, '.').nth(1).unwrap_or(program_id);

    let install_command: String = format!(
        "winget install -e --id {} --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
        &program_id
    );

    println!(
        "{} {}...",
        message::add_delimiter(
            DelimiterType::Layer1,
            "Installing".to_string(),
            Some(true),
            None,
            None
        )
        .unwrap()
        .as_str(),
        &program_name.hex(MAIN_THEME.primary)
    );

    let install_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(install_command)
        .output()
        .expect("Failed to install program via winget...");

    if !install_command_output.status.success() {
        if String::from_utf8_lossy(&install_command_output.stdout)
            .contains("Found an existing package")
        {
            message::warning(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer2,
                    "This package is already installed!".to_string(),
                    Some(true),
                    None,
                    None,
                )
                .unwrap()
                .as_str(),
            );
        } else {
            message::error(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer2Error,
                    String::from_utf8_lossy(&install_command_output.stdout).to_string(),
                    Some(true),
                    None,
                    None,
                )
                .unwrap()
                .as_str(),
            );
        }
    } else {
        message::info(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Info,
                String::from_utf8_lossy(&install_command_output.stdout).to_string(),
                Some(true),
                None,
                None,
            )
            .unwrap()
            .as_str(),
        );
    }
}

pub fn winget_remove(program_id: &str) {
    let program_name = program_id.splitn(2, '.').nth(1).unwrap_or(program_id);

    let remove_command: String = format!("remove -e --id {}", &program_id);

    println!(
        "{} {}...",
        message::add_delimiter(
            DelimiterType::Layer1,
            "Removing".to_string(),
            Some(true),
            None,
            None
        )
        .unwrap()
        .as_str(),
        &program_name.hex(MAIN_THEME.error)
    );

    let remove_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(remove_command)
        .output()
        .expect("Failed to remove program via winget...");

    if !remove_command_output.status.success() {
        message::error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "PowerShell returned an error:\n{}",
                    String::from_utf8_lossy(&remove_command_output.stdout)
                )
                .to_string(),
                Some(true),
                None,
                None,
            )
            .unwrap()
            .as_str(),
        );
    } else {
        println!(
            "{}",
            String::from_utf8_lossy(&remove_command_output.stdout).hex(MAIN_THEME.info)
        );
        message::info(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Info,
                String::from_utf8_lossy(&remove_command_output.stdout).to_string(),
                Some(true),
                None,
                None,
            )
            .unwrap()
            .as_str(),
        );
    }
}
