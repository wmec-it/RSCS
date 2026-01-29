// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/stickykeys/

use rscs::core::command::powershell;

#[allow(dead_code)]
pub fn enable() {
    powershell::default(
        "Enabling Sticky Keys on startup...",
        "Successfully enabled Sticky Keys on startup!",
        "Error enabling Sticky Keys on startup...",
        "
        try {
            $value = 510
            $Path = \"HKCU:\\Control Panel\\Accessibility\\StickyKeys\"
            Set-ItemProperty -Path $Path -Name Flags -Value $value
        } catch [System.Security.SecurityException] {
            Write-Warning \"Unable to set $Path\\$Name to $Value due to a Security Exception\"
        } catch [System.Management.Automation.ItemNotFoundException] {
            Write-Warning $psitem.Exception.ErrorRecord
        } catch {
            Write-Warning \"Unable to set $Name due to unhandled exception\"
            Write-Warning $psitem.Exception.StackTrace
        }",
    );
}

#[allow(dead_code)]
pub fn disable() {
    powershell::default(
        "Disabling Sticky Keys on startup...",
        "Successfully disabled Sticky Keys on startup!",
        "Error disabling Sticky Keys on startup...",
        "
        try {
            $value = 58
            $Path = \"HKCU:\\Control Panel\\Accessibility\\StickyKeys\"
            Set-ItemProperty -Path $Path -Name Flags -Value $value
        } catch [System.Security.SecurityException] {
            Write-Warning \"Unable to set $Path\\$Name to $Value due to a Security Exception\"
        } catch [System.Management.Automation.ItemNotFoundException] {
            Write-Warning $psitem.Exception.ErrorRecord
        } catch {
            Write-Warning \"Unable to set $Name due to unhandled exception\"
            Write-Warning $psitem.Exception.StackTrace
        }",
    );
}
