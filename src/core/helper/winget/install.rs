pub fn official_repo(program_id: &str) -> std::process::Output {
    let install_command: String = format!(
        "winget install -e --source winget --id {} --silent --disable-interactivity --accept-package-agreements --accept-source-agreements",
        &program_id
    );
    let install_command_output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(install_command)
        .output()
        .expect("Failed to install program via winget...");
    return install_command_output;
}
