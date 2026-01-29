pub fn basic(program_id: &str) -> std::process::Output {
    let remove_command: String = format!("remove -e --id {}", &program_id);
    let remove_command_output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(remove_command)
        .output()
        .expect("Failed to remove program via winget...");
    return remove_command_output;
}
