use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn default(run_message: &str, success_message: &str, error_message: &str, command: &str) {
    let run_message: String = run_message.to_string();
    let success_message: String = success_message.to_string();
    let error_message: String = error_message.to_string();
    let command: &str = command;

    println!("{}", run_message);
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        println!("{}", success_message);
    } else {
        println!("{}", error_message);
    }
}

pub fn admin(run_message: &str, success_message: &str, error_message: &str, command: &str) {
    let run_message: String = run_message.to_string();
    let success_message: String = success_message.to_string();
    let error_message: String = error_message.to_string();

    println!("{}", run_message);
    //:& Create temp files
    let temp_dir = env::temp_dir();
    let script_file: PathBuf = temp_dir.join("rscs_admin_script.ps1");
    let stderr_file: PathBuf = temp_dir.join("rscs_admin_stderr.txt");
    let exitcode_file: PathBuf = temp_dir.join("rscs_admin_exitcode.txt");

    //:& Write the command to a temp script file to avoid escaping issues
    let script_content = format!(
        r#"
            try {{
                {}
                $LASTEXITCODE | Out-File -FilePath '{}' -Encoding ASCII
            }} catch {{
                $_. Exception.Message | Out-File -FilePath '{}' -Encoding ASCII
                1 | Out-File -FilePath '{}' -Encoding ASCII
            }}
        "#,
        command,
        exitcode_file.display(),
        stderr_file.display(),
        exitcode_file.display()
    );

    //:& Write script to temp file
    if let Err(e) = fs::write(&script_file, &script_content) {
        println!("{}: {}", error_message, e);
        return;
    }

    //:& Run the script with elevation
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process -FilePath 'powershell' -ArgumentList '-ExecutionPolicy', 'Bypass', '-File', '{}' -Verb RunAs -Wait -WindowStyle Hidden",
            script_file.display()
        ))
        .output()
        .expect("Failed to run PowerShell");

    //:& Read results
    let stderr_content = fs::read_to_string(&stderr_file).unwrap_or_default();
    let exit_code: i32 = fs::read_to_string(&exitcode_file)
        .unwrap_or_else(|_| "0".to_string())
        .trim()
        .parse()
        .unwrap_or(0);

    //:& Clean up temp files
    let _ = fs::remove_file(&script_file);
    let _ = fs::remove_file(&stderr_file);
    let _ = fs::remove_file(&exitcode_file);

    if output.status.success() && exit_code == 0 && stderr_content.is_empty() {
        println!("{}", success_message);
    } else {
        println!("{}{}{}", error_message, exit_code, stderr_content);
    }
}
