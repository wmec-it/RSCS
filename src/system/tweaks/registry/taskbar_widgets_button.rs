// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/taskbarwidgets/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::admin(
        "Enabling Taskbar Widgets...",
        "Successfully enabled Taskbar Widgets!",
        "Failed to enable Taskbar Widgets...",
        "try {
            $value = 1
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name TaskbarDa -Value $value
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
        "Disabling Taskbar Widgets...",
        "Successfully disabled Taskbar Widgets!",
        "Failed to disable Taskbar Widgets...",
        "try {
            $value = 0
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name TaskbarDa -Value $value
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
