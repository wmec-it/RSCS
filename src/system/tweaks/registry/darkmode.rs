// https://winutil.christitus.com/dev/tweaks/customize-preferences/darkmode/

use crate::{AppContext, system::tweaks::templates};

#[allow(dead_code)]
pub fn enable(ctx: &mut AppContext) {
    templates::default(
        ctx,
        "Enabling dark mode...",
        "Successfully enabled dark mode!",
        "Error enabling dark mode...",
        "$Path = \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\"
        Set-ItemProperty -Path $Path -Name AppsUseLightTheme -Value 0
        Set-ItemProperty -Path $Path -Name SystemUsesLightTheme -Value 0",
    );
}

#[allow(dead_code)]
pub fn disable(ctx: &mut AppContext) {
    templates::default(
        ctx,
        "Disabling dark mode...",
        "Successfully disabled dark mode!",
        "Error disabling dark mode...",
        "$Path = \"HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\"
        Set-ItemProperty -Path $Path -Name AppsUseLightTheme -Value 1
        Set-ItemProperty -Path $Path -Name SystemUsesLightTheme -Value 1",
    );
}
