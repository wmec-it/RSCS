// DOCS: https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/display/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Enabling Display Performance Mode...",
        "Successfully enabled Display Performance Mode!",
        "Failed to enable Display Performance Mode...",
        "Set-ItemProperty -Path \"HKCU:\\Control Panel\\Desktop\" -Name \"UserPreferencesMask\" -Type Binary -Value ([byte[]](144,18,3,128,16,0,0,0))",
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Disabling Display Performance Mode...",
        "Successfully disabled Display Performance Mode!",
        "Failed to disable Display Performance Mode...",
        "Remove-ItemProperty -Path \"HKCU:\\Control Panel\\Desktop\" -Name \"UserPreferencesMask\"",
    );
}
