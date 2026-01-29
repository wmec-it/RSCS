// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/bingsearch/

use rscs::core::command::powershell;

#[allow(dead_code)]
pub fn enable() {
    powershell::default(
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
    powershell::default(
        "Disabling web search results from Bing in Start Menu search...",
        "Successfully disabled web search results from Bing in Start Menu search!",
        "Failed to disable web search results from Bing in Start Menu search...",
        "
$Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\"
        Set-ItemProperty -Path $Path -Name BingSearchEnabled -Value 0
",
    );
}
