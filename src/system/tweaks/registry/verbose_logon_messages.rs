// https://winutil.christitus.com/dev/tweaks/customize-preferences/verboselogon/

use crate::{AppContext, system::tweaks::templates};

#[allow(dead_code)]
pub fn enable(ctx: &mut AppContext) {
    templates::admin(
        ctx,
        "Enabling Verbose Logon Messages...",
        "Successfully enabled Verbose Logon Messages!",
        "Failed to enable Verbose Logon Messages...",
        "try {
            $value = 1
            $Path = \"HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System\"
            Set-ItemProperty -Path $Path -Name VerboseStatus -Value $value
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
pub fn disable(ctx: &mut AppContext) {
    templates::admin(
        ctx,
        "Disabling Verbose Logon Messages...",
        "Successfully disabled Verbose Logon Messages!",
        "Failed to disable Verbose Logon Messages...",
        "try {
            $value = 0
            $Path = \"HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System\"
            Set-ItemProperty -Path $Path -Name VerboseStatus -Value $value
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
