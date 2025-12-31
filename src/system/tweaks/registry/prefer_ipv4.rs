// https://winutil.christitus.com/dev/tweaks/essential-tweaks/ipv46/

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
            "Setting registry key to prefer IPv4 over IPv6...".to_string(),
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
            "$path = \"HKLM:\\SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters\"
$name = \"DisabledComponents\"
$value = 32
if (Get-ItemProperty -Path $path -Name $name -ErrorAction SilentlyContinue) {
    Set-ItemProperty -Path $path -Name $name -Value $value -Type DWord
} else {
    New-ItemProperty -Path $path -Name $name -PropertyType DWord -Value $value -Force
}",
        )
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                "Successfully Set registry key to prefer IPv4 over IPv6!".to_string(),
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
                    "Failed to set registry key to prefer IPv4 over IPv6...",
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
            "Reverting registry key to prefer IPv4 over IPv6 to 0 (disabled)...".to_string(),
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
            "$path = \"HKLM:\\SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters\"
$name = \"DisabledComponents\"
$value = 0
if (Get-ItemProperty -Path $path -Name $name -ErrorAction SilentlyContinue) {
    Set-ItemProperty -Path $path -Name $name -Value $value -Type DWord
} else {
    New-ItemProperty -Path $path -Name $name -PropertyType DWord -Value $value -Force
}",
        )
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                "Successfully Reverted registry key to prefer IPv4 over IPv6 to 0 (disabled)!"
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
                    "Failed to revert registry key to prefer IPv4 over IPv6 to 0 (disabled)...",
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
