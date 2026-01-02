// https://winutil.christitus.com/dev/tweaks/customize-preferences/taskview/

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

#[allow(dead_code)]
pub fn enable() {
    let run_message: String = "Enabling Taskbar Task View Button...".to_string();
    let success_message: String = "Successfully enabled Taskbar Task View Button!".to_string();
    let error_message: String = "Failed to disable Taskbar Task View Button...".to_string();
    let command: &str = "try {
       $value = 1
        $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
        Set-ItemProperty -Path $Path -Name ShowTaskViewButton -Value $value
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
    let run_message: String = "Disabling Taskbar Task View Button...".to_string();
    let success_message: String = "Successfully disabled Taskbar Task View Button!".to_string();
    let error_message: String = "Failed to Taskbar Task View Button...".to_string();
    let command: &str = "try {
       $value = 0
        $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
        Set-ItemProperty -Path $Path -Name ShowTaskViewButton -Value $value
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