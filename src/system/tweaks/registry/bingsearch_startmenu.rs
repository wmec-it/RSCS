// https://winutil.christitus.com/dev/tweaks/essential-tweaks/powershell7tele/

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

#[allow(dead_code)]
pub fn enable() {
    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            "Enabling web search results from Bing in Start Menu search...".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(
            "
$Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\"
        Set-ItemProperty -Path $Path -Name BingSearchEnabled -Value 1
",
        )
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                "Successfully enabled web search results from Bing in Start Menu search!"
                    .to_string(),
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
                    "Failed to enable web search results from Bing in Start Menu search...",
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
    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            "Disabling web search results from Bing in Start Menu search...".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(
            "
$Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\"
        Set-ItemProperty -Path $Path -Name BingSearchEnabled -Value 0
",
        )
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                "Successfully disabled web search results from Bing in Start Menu search!"
                    .to_string(),
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
                    "Failed to disable web search results from Bing in Start Menu search...",
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
