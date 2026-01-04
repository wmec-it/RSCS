// https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/removecopilot/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Enabling Microsoft Copilot...",
        "Successfully enabled Microsoft Copilot!",
        "Failed to enable Microsoft Copilot",
        "dism /online /add-package /package-name:Microsoft.Windows.Copilot",
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Removing Microsoft Copilot...",
        "Successfully removed Microsoft Copilot!",
        "Failed to remove Microsoft Copilot",
        "dism /online /remove-package /package-name:Microsoft.Windows.Copilot",
    );
}
