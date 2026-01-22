//:& https://winutil.christitus.com/dev/tweaks/essential-tweaks/ipv46/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::admin(
        "Setting registry key to prefer IPv4 over IPv6...",
        "Successfully Set registry key to prefer IPv4 over IPv6!",
        "Failed to set registry key to prefer IPv4 over IPv6...",
        "$path = \"HKLM:\\SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters\"
        $name = \"DisabledComponents\"
        $value = 32
        if (Get-ItemProperty -Path $path -Name $name -ErrorAction SilentlyContinue) {
            Set-ItemProperty -Path $path -Name $name -Value $value -Type DWord
        } else {
            New-ItemProperty -Path $path -Name $name -PropertyType DWord -Value $value -Force
        }",
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::admin(
        "Reverting registry key to prefer IPv4 over IPv6 to 0 (disabled)...",
        "Successfully Reverted registry key to prefer IPv4 over IPv6 to 0 (disabled)!",
        "Failed to revert registry key to prefer IPv4 over IPv6 to 0 (disabled)...",
        "$path = \"HKLM:\\SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters\"
        $name = \"DisabledComponents\"
        $value = 0
        if (Get-ItemProperty -Path $path -Name $name -ErrorAction SilentlyContinue) {
            Set-ItemProperty -Path $path -Name $name -Value $value -Type DWord
        } else {
            New-ItemProperty -Path $path -Name $name -PropertyType DWord -Value $value -Force
        }",
    );
}
