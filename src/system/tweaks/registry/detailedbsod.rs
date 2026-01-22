// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/detailedbsod/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::admin(
        "Enabling detailed BSOD...",
        "Successfully enabled detailed BSOD!",
        "Failed to enable detailed BSOD...",
        "try {
            $value = 1
            $Path = \"HKLM:\\SYSTEM\\CurrentControlSet\\Control\\CrashControl\"
            Set-ItemProperty -Path $Path -Name DisplayParameters -Value $value
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
    templates::admin(
        "Disabling detailed BSOD...",
        "Successfully disabled detailed BSOD!",
        "Failed to disable detailed BSOD...",
        "try {
            $value = 0
            $Path = \"HKLM:\\SYSTEM\\CurrentControlSet\\Control\\CrashControl\"
            Set-ItemProperty -Path $Path -Name DisplayParameters -Value $value
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
