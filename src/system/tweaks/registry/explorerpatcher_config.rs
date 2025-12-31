use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

#[allow(dead_code)]
pub fn enable() {
    let run_message: String = "Configuring ExplorerPatcher...".to_string();
    let success_message: String = "Successfully configured ExplorerPatcher!".to_string();
    let error_message: String = "Error configuring ExplorerPatcher...".to_string();
    let command: &str = "$registryEntries = @(
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"ImportOK\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"OldTaskbar\"; Value = 2},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\"; Name = \"SearchboxTaskbarMode\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"ShowTaskViewButton\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"OrbStyle\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"OldTaskbarAl\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"MMOldTaskbarAl\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"TaskbarGlomLevel\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"MMTaskbarGlomLevel\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"TaskbarSmallIcons\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"SkinMenus\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"CenterMenus\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"FlyoutMenus\"; Value = 1},
    @{Path = \"HKCU:\\Software\\Microsoft\\TabletTip\\1.7\"; Name = \"TipbandDesiredVisibility\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"ShowSecondsInSystemClock\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"HideControlCenterButton\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"TaskbarSD\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"TrayOverflowStyle\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"ReplaceNetwork\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Control Panel\\Settings\\Network\"; Name = \"ReplaceVan\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows NT\\CurrentVersion\\MTCUVC\"; Name = \"EnableMtcUvc\"; Value = 1},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\ImmersiveShell\"; Name = \"UseWin32TrayClockExperience\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\ImmersiveShell\"; Name = \"UseWin32BatteryFlyout\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"IMEStyle\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"LegacyFileTransferDialog\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UseClassicDriveGrouping\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DisableImmersiveContextMenu\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"ShrinkExplorerAddressBar\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"HideExplorerSearchBar\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"HideIconAndTitleInExplorer\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"MicaEffectOnTitlebar\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"Start_ShowClassicMode\"; Value = 1},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"TaskbarAl\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartPage\"; Name = \"MonitorOverride\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"XamlSounds\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"Language\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"IncludeWallpaper\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"PrimaryOnly\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"PerMonitor\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"SwitcherIsPerApplication\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"NoPerApplicationList\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"Theme\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"ColorScheme\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"CornerPreference\"; Value = 2},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"RowHeight\"; Value = 230},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"MaxWidth\"; Value = 80},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"MaxHeight\"; Value = 80},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"MasterPadding\"; Value = 20},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"ShowDelay\"; Value = 100},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\\sws\"; Name = \"ScrollWheelBehavior\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"LastSectionInProperties\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"ClockFlyoutOnWinC\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"ToolbarSeparators\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"PropertiesInWinX\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"NoMenuAccelerator\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DisableOfficeHotkeys\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DisableWinFHotkey\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DisableAeroSnapQuadrants\"; Value = 0},
    @{Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"; Name = \"Start_PowerButtonAction\"; Value = 2},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DoNotRedirectSystemToSettingsApp\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DoNotRedirectProgramsAndFeaturesToSettingsApp\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DoNotRedirectDateAndTimeToSettingsApp\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"DoNotRedirectNotificationIconsToSettingsApp\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UpdatePolicy\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UpdatePreferStaging\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UpdateAllowDowngrades\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UpdateUseLocal\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UpdateURL\"; Value = \"\"},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"UpdateURLStaging\"; Value = \"\"},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"AllocConsole\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"Memcheck\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"TaskbarAutohideOnDoubleClick\"; Value = 0},
    @{Path = \"HKCU:\\Control Panel\\Desktop\"; Name = \"PaintDesktopVersion\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"ClassicThemeMitigations\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"NoPropertiesInContextMenu\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"EnableSymbolDownload\"; Value = 1},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"PinnedItemsActAsQuickLaunch\"; Value = 0},
    @{Path = \"HKCU:\\Software\\ExplorerPatcher\"; Name = \"RemoveExtraGapAroundPinnedItems\"; Value = 0}
)

foreach ($entry in $registryEntries) {
    Set-ItemProperty -Path $entry.Path -Name $entry.Name -Value $entry.Value
    Write-Host \"Added registry entry: $($entry.Path)\\$($entry.Name) = $($entry.Value)\"
}";

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
                    "{}\\nExit Code: {:?}\\n{}",
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

    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            "Restarting explorer...".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Stop-Process -Name explorer -Force; Start-Process explorer")
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                "Successfully restarted Explorer!".to_string(),
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
                    "{}\\nExit Code: {:?}\\n{}",
                    "Failed to restart Explorer...",
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
    // TODO: Add disabling (revert to defaults)
}
