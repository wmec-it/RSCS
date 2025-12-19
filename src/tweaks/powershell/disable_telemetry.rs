use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

pub fn main() {
    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            "Disabling Powershell 7 Telemetry...".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("[Environment]::SetEnvironmentVariable('POWERSHELL_TELEMETRY_OPTOUT', '1', 'Machine')")
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                "Disabled Powershell 7 Telemetry!".to_string(),
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
                    "Failed to Disable Powershell 7 Telemetry...",
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
