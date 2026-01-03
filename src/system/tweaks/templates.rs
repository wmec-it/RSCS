use std::process::Command;

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub fn default(run_message: &str, success_message: &str, error_message: &str, command: &str) {
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

pub fn admin(run_message: &str, success_message: &str, error_message: &str, command: &str) {
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