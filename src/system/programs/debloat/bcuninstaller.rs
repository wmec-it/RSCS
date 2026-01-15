// https://download.fosshub.com/Protected/expiretime=1768524194;badurl=aHR0cHM6Ly93d3cuZm9zc2h1Yi5jb20vQnVsay1DcmFwLVVuaW5zdGFsbGVyLmh0bWw=/b45e73fa40a07dcc8ea96efc41dffc9f5d776062b581bffe9170e3eab0bd63d4/5b964d315dc7e865ea596350/67b79fafcb7597043c5a3f90/BCUninstaller_5.8.3_setup.exe

use std::process::Command;

#[allow(dead_code)]
pub fn setup() -> () {
    set_env_vars();
}

fn set_env_vars() -> () {
    Command::new("powershell").arg("-Command").arg("[System.Environment]::SetEnvironmentVariable(\"PATH\", $env:PATH + \";C:\\Program Files\\BCUninstaller\", [System.EnvironmentVariableTarget]::Machine)").output().unwrap();
}
