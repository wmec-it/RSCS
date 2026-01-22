// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/taskbarsearch/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Enabling Taskbar Search button...",
        "Successfully enabled Taskbar Search button!",
        "Failed to enable Taskbar Search button...",
        "try {
            $value = 1
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\\\"
            Set-ItemProperty -Path $Path -Name SearchboxTaskbarMode -Value $value
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
    templates::default(
        "Disabling Taskbar Search button...",
        "Successfully disabled Taskbar Search button!",
        "Failed to disable Taskbar Search button...",
        "try {
            $value = 0
            $Path = \"HKCU:\\\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\\\"
            Set-ItemProperty -Path $Path -Name SearchboxTaskbarMode -Value $value
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
