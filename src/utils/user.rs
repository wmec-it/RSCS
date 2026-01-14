use crate::AppContext;
use crate::conf::enums::{DelimiterType, MessageType};
use crate::utils::message::{add_delimiter, error, info};
use std::process::Command;

pub fn enable_sudo(ctx: &mut AppContext) {
    let enable_sudo_command_output = Command::new("powershell")
        .arg("-Command")
        .arg("sudo config --enable normal")
        .output()
        .expect("Failed to enable Sudo...");

    if !enable_sudo_command_output.status.success() {
        error(
            ctx,
            MessageType::Print,
            add_delimiter(
                DelimiterType::Layer1Error,
                String::from_utf8_lossy(&enable_sudo_command_output.stdout).to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else {
        info(
            ctx,
            MessageType::Print,
            add_delimiter(
                DelimiterType::Layer1Info,
                String::from_utf8_lossy(&enable_sudo_command_output.stdout).to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
}
