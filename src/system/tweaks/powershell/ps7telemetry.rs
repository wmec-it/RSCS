// DOCS: https://winutil.christitus.com/dev/tweaks/essential-tweaks/powershell7tele/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Disabling Powershell 7 Telemetry...",
        "Successfully Disabled Powershell 7 Telemetry!, error_message",
        "Failed to Disable Powershell 7 Telemetry...",
        "[Environment]::SetEnvironmentVariable('POWERSHELL_TELEMETRY_OPTOUT', '1', 'Machine')",
    );
}

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Enabling Powershell 7 Telemetry...",
        "Successfully Enabled Powershell 7 Telemetry!",
        "Failed to Enable Powershell 7 Telemetry...",
        "[Environment]::SetEnvironmentVariable('POWERSHELL_TELEMETRY_OPTOUT', '', 'Machine')",
    );
}
