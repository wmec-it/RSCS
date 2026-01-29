// DOCS: https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/disablenotifications/

use rscs::core::command::powershell;

#[allow(dead_code)]
pub fn enable() {
    powershell::default(
        "Enabling Notification Center and Calendar...",
        "Successfully enabled Notification Center and Calendar!",
        "Failed to enable Notification Center and Calendar...",
        "
        New-ItemProperty -Path \"HKCU:\\Software\\Policies\\Microsoft\\Windows\\Explorer\" -Name \"DisableNotificationCenter\" -Value 0 -Force
        New-ItemProperty -Path \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\PushNotifications\" -Name \"ToastEnabled\" -Value 1 -Force
        "
    );
}

#[allow(dead_code)]
pub fn disable() {
    powershell::default(
        "Disabling Notification Center and Calendar...",
        "Successfully disabled Notification Center and Calendar!",
        "Failed to disable Notification Center and Calendar...",
        "
        New-ItemProperty -Path \"HKCU:\\Software\\Policies\\Microsoft\\Windows\\Explorer\" -Name \"DisableNotificationCenter\" -Value 1 -Force
        New-ItemProperty -Path \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\PushNotifications\" -Name \"ToastEnabled\" -Value 0 -Force
        "
    );
}
