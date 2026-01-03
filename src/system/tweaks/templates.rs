use base64::{Engine as _, engine::general_purpose};
use std::process::Command;

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub fn default(run_message: &str, success_message: &str, error_message: &str, command: &str) {
    run(run_message, success_message, error_message, command, false);
}

pub fn admin(run_message: &str, success_message: &str, error_message: &str, command: &str) {
    run(run_message, success_message, error_message, command, true);
}

fn encode_ps(command: &str) -> String {
    let utf16: Vec<u16> = command.encode_utf16().collect();
    let bytes: Vec<u8> = utf16.iter().flat_map(|c| c.to_le_bytes()).collect();

    general_purpose::STANDARD.encode(bytes)
}

fn run(
    run_message: &str,
    success_message: &str,
    error_message: &str,
    command: &str,
    as_admin: bool,
) {
    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            run_message.to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    let output = if as_admin {
        let encoded = encode_ps(command);

        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(format!(
                "Start-Process powershell -Verb RunAs -ArgumentList '-NoProfile -EncodedCommand {}'",
                encoded
            ))
            .output()
            .expect("Failed to start elevated PowerShell")
    } else {
        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(command)
            .output()
            .expect("Failed to run PowerShell")
    };

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                success_message.to_string(),
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
