// DOCS: https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/removecopilot/
// DOCS: https://github.com/ChrisTitusTech/winutil/blob/main/config/tweaks.json
// DOCS: -    WPFTweaksRemoveCopilot

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::admin(
        "Enabling Microsoft Copilot...",
        "Successfully enabled Microsoft Copilot!",
        "Failed to enable Microsoft Copilot",
        "winget install --name Copilot --source msstore --accept-package-agreements --accept-source-agreements --silent",
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::admin(
        "Removing Microsoft Copilot...",
        "Successfully removed Microsoft Copilot!",
        "Failed to remove Microsoft Copilot",
        "
        Get-AppxPackage -AllUsers *Copilot* | Remove-AppxPackage -AllUsers
        Get-AppxPackage -AllUsers Microsoft.MicrosoftOfficeHub | Remove-AppxPackage -AllUsers
        New-ItemProperty -Path \"HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot\" -Name \"TurnOffWindowsCopilot\" -Value \"1\" -PropertyType \"DWord\" -Force
        New-ItemProperty -Path \"HKCU:\\Software\\Policies\\Microsoft\\Windows\\WindowsCopilot\" -Name \"TurnOffWindowsCopilot\" -Value \"1\" -PropertyType \"DWord\" -Force
        New-ItemProperty -Path \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"ShowCopilotButton\" -Value \"0\" -PropertyType \"DWord\" -Force
        New-ItemProperty -Path \"HKLM:\\SOFTWARE\\Microsoft\\Windows\\Shell\\Copilot\" -Name \"IsCopilotAvailable\" -Value \"0\" -PropertyType \"DWord\" -Force
        New-ItemProperty -Path \"HKLM:\\SOFTWARE\\Microsoft\\Windows\\Shell\\Copilot\" -Name \"CopilotDisabledReason\" -Value \"IsEnabledForGeographicRegionFailed\" -PropertyType \"String\" -Force
        New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\WindowsCopilot\" -Name \"AllowCopilotRuntime\" -Value \"0\" -PropertyType \"DWord\" -Force
        New-ItemProperty -Path \"HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Shell Extensions\\Blocked\" -Name \"{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}\" -Value \"\" -PropertyType \"String\" -Force
        New-ItemProperty -Path \"HKLM:\\SOFTWARE\\Microsoft\\Windows\\Shell\\Copilot\\BingChat\" -Name \"IsUserEligible\" -Value \"0\" -PropertyType \"DWord\" -Force
        ",
    );
}
