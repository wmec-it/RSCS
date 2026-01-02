use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    templates::default(
        "Enabling End Task with right click on taskbar...",
        "Successfully Enabled End Task with right click on taskbar!",
        "Failed to Enable End Task with right click on taskbar...",
        "$path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\TaskbarDeveloperSettings\"
      $name = \"TaskbarEndTask\"
      $value = 1
      if (-not (Test-Path $path)) {
        New-Item -Path $path -Force | Out-Null
      }
      New-ItemProperty -Path $path -Name $name -PropertyType DWord -Value $value -Force | Out-Null"
    );
}

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Disabling End Task with right click on taskbar...",
        "Successfully Disabled End Task with right click on taskbar!",
        "Failed to Disable End Task with right click on taskbar...",
        "$path = \"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\TaskbarDeveloperSettings\"
      $name = \"TaskbarEndTask\"
      $value = 0
      if (-not (Test-Path $path)) {
        New-Item -Path $path -Force | Out-Null
      }
      New-ItemProperty -Path $path -Name $name -PropertyType DWord -Value $value -Force | Out-Null"
    );
}
