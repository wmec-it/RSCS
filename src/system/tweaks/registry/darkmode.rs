// Link to script (if applicable)

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

#[allow(dead_code)]
pub fn enable() {
    let run_message: String = "Enabling dark mode...".to_string();
    let success_message: String = "Successfully enabled dark mode!".to_string();
    let error_message: String = "Error enabling dark mode...".to_string();
    let command: &str =
        "$Path = \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\"
        Set-ItemProperty -Path $Path -Name AppsUseLightTheme -Value 0
        Set-ItemProperty -Path $Path -Name SystemUsesLightTheme -Value 0";

    message::normal(
        MessageType::Print,
        message::add_delimiter(DelimiterType::Layer1, run_message, Some(true), None, None)
            .unwrap()
            .as_str(),
    );
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                success_message,
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else {
        message::error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "{}\nExit Code: {:?}\n{}",
                    error_message,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                ),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
}

#[allow(dead_code)]
pub fn disable() {
    let run_message: String = "Disabling dark mode...".to_string();
    let success_message: String = "Successfully disabled dark mode!".to_string();
    let error_message: String = "Error disabling dark mode...".to_string();
    let command: &str =
        "$Path = \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\"
        Set-ItemProperty -Path $Path -Name AppsUseLightTheme -Value 1
        Set-ItemProperty -Path $Path -Name SystemUsesLightTheme -Value 1";

    message::normal(
        MessageType::Print,
        message::add_delimiter(DelimiterType::Layer1, run_message, Some(true), None, None)
            .unwrap()
            .as_str(),
    );
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                success_message,
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else {
        message::error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "{}\nExit Code: {:?}\n{}",
                    error_message,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                ),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
}
