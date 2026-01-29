pub fn enable() -> std::process::Output {
    let enable_sudo_command_output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg("sudo config --enable normal")
        .output()
        .expect("Failed to enable Sudo...");
    return enable_sudo_command_output;
}
