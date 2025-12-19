// https://winutil.christitus.com/dev/tweaks/essential-tweaks/powershell7/

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
    winget::winget_install,
};
use std::process::Command;

pub fn full() {
    #[allow(unused_mut)]
    let mut has_failed_install: bool = false;
    let mut has_failed_customization: bool = false;

    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Setting Powershell 7 as default".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    winget_install("Microsoft.PowerShell");

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer2Success,
            "Finished Installing Powershell 7, or it is already installed!".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    let check_wt_output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Command \"wt\" -ErrorAction SilentlyContinue")
        .output()
        .expect("Windows Terminal not installed...");
    if String::from_utf8_lossy(&check_wt_output.stdout).to_string() == "" {
        has_failed_customization = true;
        message::error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                "Windows Terminal not installed. Skipping Terminal preference...".to_string(),
                Some(true),
                None,
                None,
            )
            .unwrap()
            .as_str(),
        );
    } else {
        let check_ps7_settings_file_exists_output = Command::new("powershell")
            .arg("-Command")
            .arg(
                "Test-Path -Path $env:LOCALAPPDATA\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json"
            )
            .output()
            .expect("sadasdsd");
        if String::from_utf8_lossy(&check_ps7_settings_file_exists_output.stdout)
            .to_string()
            .contains("True")
        {
            message::normal(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer1,
                    "Windows Terminal is installed. Continuing...".to_string(),
                    Some(true),
                    None,
                    None,
                )
                .unwrap()
                .as_str(),
            );

            message::normal(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer1,
                    "Settings file found.".to_string(),
                    Some(true),
                    None,
                    None,
                )
                .unwrap()
                .as_str(),
            );

            message::info(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer1Info,
                    "Updating default Windows Terminal profile...".to_string(),
                    Some(true),
                    None,
                    None,
                )
                .unwrap()
                .as_str(),
            );

            Command::new("powershell")
                .arg("-NoProfile")
                .arg("-Command")
                .arg(
                    "$settingsPath=\"$env:LOCALAPPDATA\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json\"; \
         $targetTerminalName='PowerShell'; \
         if(Test-Path $settingsPath){ \
            $settingsContent=Get-Content $settingsPath|ConvertFrom-Json; \
            if($p=$settingsContent.profiles.list|Where-Object name -eq $targetTerminalName){ \
                $settingsContent.defaultProfile=$p.guid; \
                $settingsContent|ConvertTo-Json -Depth 100|Set-Content $settingsPath \
            } \
         }"
                )
                .output()
                .expect("Error...");

            message::success(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer2Success,
                    "Finished Updating default Windows Terminal Profile Successfully!".to_string(),
                    Some(true),
                    None,
                    None,
                )
                .unwrap()
                .as_str(),
            );
        } else {
            has_failed_customization = true;
            message::error(
                MessageType::Print,
                message
                    ::add_delimiter(
                        DelimiterType::Layer2Error,
                        "Windows Terminal Settings file not found at $env:LOCALAPPDATA\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json".to_string(),
                        Some(true),
                        None,
                        None
                    )
                    .unwrap()
                    .as_str()
            );
        }
    }

    if !has_failed_customization && !has_failed_install {
        message::success(MessageType::Print, "|");
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer1Success,
                "Successfully Installed and set Powershell 7 as default in Windows Terminal!"
                    .to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else if !has_failed_customization && has_failed_install {
        message::error(MessageType::Print, "|");
        message::error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer1Error,
                "Failed Installing and setting Powershell 7 as default in Windows Terminal..."
                    .to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else if has_failed_customization && !has_failed_install {
        message::error(MessageType::Print, "|");
        message::error(
            MessageType::Print,
            message
                ::add_delimiter(
                    DelimiterType::Layer1Error,
                    "Successfully Installed Powershell 7, but failed setting it as default in Windows Terminal...".to_string(),
                    Some(true),
                    None,
                    Some(true)
                )
                .unwrap()
                .as_str()
        );
    }
}
