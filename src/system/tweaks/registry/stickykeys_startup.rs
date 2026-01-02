// https://winutil.christitus.com/dev/tweaks/customize-preferences/stickykeys/

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

#[allow(dead_code)]
pub fn enable() {
    let run_message: String = "Enabling Sticky Keys on startup...".to_string();
    let success_message: String = "Successfully enabled Sticky Keys on startup!".to_string();
    let error_message: String = "Error enabling Sticky Keys on startup...".to_string();
    let command: &str = "
    try {
        $value = 510
        $Path = \"HKCU:\\Control Panel\\Accessibility\\StickyKeys\"
        Set-ItemProperty -Path $Path -Name Flags -Value $value
    } catch [System.Security.SecurityException] {
        Write-Warning \"Unable to set $Path\\$Name to $Value due to a Security Exception\"
    } catch [System.Management.Automation.ItemNotFoundException] {
        Write-Warning $psitem.Exception.ErrorRecord
    } catch {
        Write-Warning \"Unable to set $Name due to unhandled exception\"
        Write-Warning $psitem.Exception.StackTrace
    }";

    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            run_message,
            Some(true),
            None,
            None,
        )
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
    let run_message: String = "Disabling Sticky Keys on startup...".to_string();
    let success_message: String = "Successfully disabled Sticky Keys on startup!".to_string();
    let error_message: String = "Error disabling Sticky Keys on startup...".to_string();
    let command: &str = "
    try {
        $value = 58
        $Path = \"HKCU:\\Control Panel\\Accessibility\\StickyKeys\"
        Set-ItemProperty -Path $Path -Name Flags -Value $value
    } catch [System.Security.SecurityException] {
        Write-Warning \"Unable to set $Path\\$Name to $Value due to a Security Exception\"
    } catch [System.Management.Automation.ItemNotFoundException] {
        Write-Warning $psitem.Exception.ErrorRecord
    } catch {
        Write-Warning \"Unable to set $Name due to unhandled exception\"
        Write-Warning $psitem.Exception.StackTrace
    }";

    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            run_message,
            Some(true),
            None,
            None,
        )
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