// https://winutil.christitus.com/dev/tweaks/customize-preferences/taskbaralignment/

use crate::{AppContext, system::tweaks::templates};

#[allow(dead_code)]
pub fn center(ctx: &mut AppContext) {
    templates::default(
        ctx,
        "Setting Taskbar alignment to center...",
        "Successfully set Taskbar alignment to center!",
        "Failed to set Taskbar alignment to center...",
        "New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"TaskbarAl\" -Value 1 -Force",
    );
}

#[allow(dead_code)]
pub fn left(ctx: &mut AppContext) {
    templates::default(
        ctx,
        "Setting Taskbar alignment to the left...",
        "Successfully set Taskbar alignment to the left!",
        "Failed to set Taskbar alignment to the left...",
        "New-ItemProperty -Path \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" -Name \"TaskbarAl\" -Value 0 -Force",
    );
}
