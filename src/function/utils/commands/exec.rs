use std::process::Command;

#[allow(dead_code)]
pub fn ps1(path: &str) -> std::io::Result<()> {
    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.trim().is_empty() {
        println!("\x1b[1mOutput:\x1b[0m\n\x1b[51m{}\x1b[0m", stdout.trim());
        return Ok(());
    }

    if !stderr.trim().is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("\n\x1b[91mPowerShell failed:\n{}\x1b[0m", stderr.trim()),
        ));
    }

    Ok(())
}

// TODO: Fix this shit / make it work (it doesn't actually run as admin yet)
#[allow(dead_code)]
pub fn bat(path: &str) -> std::io::Result<Vec<String>> {
    let output = Command::new("cmd")
        .args(["/C", &path])
        .output()
        .expect("Failed to run batch file");

    let out: Vec<String> = vec![
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ];

    Ok(out)
}
