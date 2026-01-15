pub mod remove;

use std::process::Command;

// TODO: Maybe hardcode path? Idk tho

#[allow(dead_code)]
pub fn setup() -> () {
    set_env_vars();
}

fn set_env_vars() -> () {
    Command::new("powershell").arg("-Command").arg("[System.Environment]::SetEnvironmentVariable(\"PATH\", $env:PATH + \";C:\\Program Files\\BCUninstaller\", [System.EnvironmentVariableTarget]::Machine)").output().unwrap();
}
