// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/taskbaralignment/

use rscs::core::command::powershell;

#[allow(dead_code)]
pub fn center() {
    powershell::default(
        "Setting Taskbar alignment to center...",
        "Successfully set Taskbar alignment to center!",
        "Failed to set Taskbar alignment to center...",
        "New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"TaskbarAl\" -Value 1 -Force",
    );
}

#[allow(dead_code)]
pub fn left() {
    powershell::default(
        "Setting Taskbar alignment to the left...",
        "Successfully set Taskbar alignment to the left!",
        "Failed to set Taskbar alignment to the left...",
        "New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"TaskbarAl\" -Value 0 -Force",
    );
}
