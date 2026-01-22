//:& https://winutil.christitus.com/dev/tweaks/customize-preferences/taskview/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::admin(
        "Enabling Taskbar Task View Button...",
        "Successfully enabled Taskbar Task View Button!",
        "Failed to disable Taskbar Task View Button...",
        "
        try {
            $value = 1
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name ShowTaskViewButton -Value $value
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
        "Disabling Taskbar Task View Button...",
        "Successfully disabled Taskbar Task View Button!",
        "Failed to disable Taskbar Task View Button...",
        "
        try {
            $value = 0
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name ShowTaskViewButton -Value $value
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
