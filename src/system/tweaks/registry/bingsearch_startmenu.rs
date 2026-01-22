// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/bingsearch/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Enabling web search results from Bing in Start Menu search...",
        "Successfully enabled web search results from Bing in Start Menu search!",
        "Failed to enable web search results from Bing in Start Menu search...",
        "
$Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\"
        Set-ItemProperty -Path $Path -Name BingSearchEnabled -Value 1
",
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Disabling web search results from Bing in Start Menu search...",
        "Successfully disabled web search results from Bing in Start Menu search!",
        "Failed to disable web search results from Bing in Start Menu search...",
        "
$Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\"
        Set-ItemProperty -Path $Path -Name BingSearchEnabled -Value 0
",
    );
}
