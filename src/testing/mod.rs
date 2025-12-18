use std::process::Command;

pub fn testing() {
    let out = Command::new("powershell").arg("-Command").arg("$settingsContent = Get-Content -Path $settingsPath | ConvertFrom-Json; $ps7Profile = $settingsContent.profiles.list | Where-Object { $_.name -eq $targetTerminalName }; if ($ps7Profile) {$settingsContent.defaultProfile = $ps7Profile.guid; $updatedSettings = $settingsContent | ConvertTo-Json -Depth 100;Set-Content -Path $settingsPath -Value $updatedSettings}").output().expect("sadasdsd");
    println!("{:#?}", String::from_utf8_lossy(&out.stdout).to_string());
    // if String::from_utf8_lossy(&out.stdout).to_string() == "" {
    //     println!("Meow");
    // }
}
