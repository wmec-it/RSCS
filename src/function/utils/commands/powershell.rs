#[allow(dead_code)]
pub fn refreshenv() -> () {
    std::process::Command::new("powershell").arg("-Command").arg("$env:Path = [System.Environment]::GetEnvironmentVariable(\"Path\",\"Machine\") + \";\" + [System.Environment]::GetEnvironmentVariable(\"Path\",\"User\")").output().unwrap();
}
