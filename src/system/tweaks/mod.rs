use std::process::Command;

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub mod power;
pub mod powershell;
pub mod registry;

pub fn run_tweaks() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Running Tweaks...\n|".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    // Powershell Tweaks
    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Running Powershell tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    powershell::ps7::full();
    powershell::ps7telemetry::disable();

    // Registry Tweaks
    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Running Registry tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    registry::rclick_end_task::enable();
    registry::prefer_ipv4::enable();
    registry::bingsearch_startmenu::disable();
    registry::darkmode::enable();
    registry::explorerpatcher_config::enable();
    registry::taskview_button::disable();
    registry::stickykeys_startup::disable();

    // Power Tweaks
    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Running Power tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    power::ultimate_powerplan::enable();
}

pub fn tweak(run_message: &str, success_message: &str, error_message: &str, command: &str) {
    let run_message: String = run_message.to_string();
    let success_message: String = success_message.to_string();
    let error_message: String = error_message.to_string();
    let command: &str = command;

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
