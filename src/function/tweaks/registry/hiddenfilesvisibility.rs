// DOCS: https://winutil.christitus.com/dev/tweaks/customize-preferences/hiddenfiles/

use rscs::core::command::powershell;

#[allow(dead_code)]
pub fn enable() {
    powershell::default(
        "Making hidden files visible...",
        "Successfully made hidden files visible!",
        "Failed to make hidden files visible...",
        "try {
            $value = 1
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name Hidden -Value $value
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
        "Making hidden files hidden...",
        "Successfully made hidden files hidden!",
        "Failed to make hidden files hidden...",
        "try {
            $value = 0
            $Path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\"
            Set-ItemProperty -Path $Path -Name Hidden -Value $value
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
