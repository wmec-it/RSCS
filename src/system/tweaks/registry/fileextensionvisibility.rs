// Link to script (if applicable)

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::admin(
        "Making file extensions visible...",
        "Successfully made file extensions visible!",
        "Failed to make file extensions visible...",
        "try {
            $value = 0
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name HideFileExt -Value $value
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
        "Making file extensions hidden...",
        "Successfully made file extensions hidden!",
        "Failed to make file extensions hidden...",
        "try {
            $value = 1
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name HideFileExt -Value $value
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
