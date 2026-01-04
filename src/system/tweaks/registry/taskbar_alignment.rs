// https://winutil.christitus.com/dev/tweaks/customize-preferences/taskbaralignment/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn center() {
    templates::default(
        "Setting Taskbar alignment to center...",
        "Successfully set Taskbar alignment to center!",
        "Failed to set Taskbar alignment to center...",
        "New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"TaskbarAl\" -Value 1 -Force",
    );
}

#[allow(dead_code)]
pub fn left() {
    templates::default(
        "Setting Taskbar alignment to the left...",
        "Successfully set Taskbar alignment to the left!",
        "Failed to set Taskbar alignment to the left...",
        "New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"TaskbarAl\" -Value 0 -Force",
    );
}
