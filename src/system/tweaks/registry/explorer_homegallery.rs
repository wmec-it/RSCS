// DOCS: https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/removehomegallery/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Adding Home and Gallery to Explorer...",
        "Successfully added Home and Gallery to Explorer!",
        "Failed to add Home and Gallery to Explorer...",
        "
        REG ADD \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}\" /f /ve /t REG_SZ /d \"{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}\"
        REG ADD \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{f874310e-b6b7-47dc-bc84-b9e6b38f5903}\" /f /ve /t REG_SZ /d \"CLSID_MSGraphHomeFolder\"
        REG DELETE \"HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" /f /v \"LaunchTo\"
        "
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Removing Home and Gallery from Explorer...",
        "Successfully removed Home and Gallery from Explorer!",
        "Failed to remove Home and Gallery from Explorer...",
        "
        REG DELETE \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}\" /f
        REG DELETE \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{f874310e-b6b7-47dc-bc84-b9e6b38f5903}\" /f
        REG ADD \"HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" /f /v \"LaunchTo\" /t REG_DWORD /d \"1\"
        "
    );
}
