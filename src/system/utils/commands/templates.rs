use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::AppContext;
use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub fn default(
    ctx: &mut AppContext,
    run_message: &str,
    success_message: &str,
    error_message: &str,
    command: &str,
) {
    let new_value: u8 = ctx.pbl + 1;
    ctx.set_pbl_value(new_value);

    let run_message: String = run_message.to_string();
    let success_message: String = success_message.to_string();
    let error_message: String = error_message.to_string();
    let command: &str = command;

    message::normal(
        ctx,
        MessageType::Print,
        message::add_delimiter(DelimiterType::Layer1, run_message, Some(true), None, None)
            .unwrap()
            .as_str(),
    );
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .output()
        .expect("Failed to run PowerShell");

    if output.status.success() {
        message::success(
            ctx,
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                success_message,
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else {
        message::error(
            ctx,
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "{}\nExit Code: {:?}\n{}",
                    error_message,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                ),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
    ctx.pb.inc(1);
}

pub fn admin(
    ctx: &mut AppContext,
    run_message: &str,
    success_message: &str,
    error_message: &str,
    command: &str,
) {
    let new_value: u8 = ctx.pbl + 1;
    ctx.set_pbl_value(new_value);

    let run_message: String = run_message.to_string();
    let success_message: String = success_message.to_string();
    let error_message: String = error_message.to_string();

    message::normal(
        ctx,
        MessageType::Print,
        message::add_delimiter(DelimiterType::Layer1, run_message, Some(true), None, None)
            .unwrap()
            .as_str(),
    );

    // Create temp files
    let temp_dir = env::temp_dir();
    let script_file: PathBuf = temp_dir.join("rscs_admin_script.ps1");
    let stderr_file: PathBuf = temp_dir.join("rscs_admin_stderr.txt");
    let exitcode_file: PathBuf = temp_dir.join("rscs_admin_exitcode.txt");

    // Write the command to a temp script file to avoid escaping issues
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

    // Write script to temp file
    if let Err(e) = fs::write(&script_file, &script_content) {
        message::error(
            ctx,
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!("{}\nFailed to write temp script:  {}", error_message, e),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
        return;
    }

    // Run the script with elevation
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process -FilePath 'powershell' -ArgumentList '-ExecutionPolicy', 'Bypass', '-File', '{}' -Verb RunAs -Wait -WindowStyle Hidden",
            script_file.display()
        ))
        .output()
        .expect("Failed to run PowerShell");

    // Read results
    let stderr_content = fs::read_to_string(&stderr_file).unwrap_or_default();
    let exit_code: i32 = fs::read_to_string(&exitcode_file)
        .unwrap_or_else(|_| "0".to_string())
        .trim()
        .parse()
        .unwrap_or(0);

    // Clean up temp files
    let _ = fs::remove_file(&script_file);
    let _ = fs::remove_file(&stderr_file);
    let _ = fs::remove_file(&exitcode_file);

    if output.status.success() && exit_code == 0 && stderr_content.is_empty() {
        message::success(
            ctx,
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Success,
                success_message,
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else {
        message::error(
            ctx,
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "{}\nExit Code: {}\n{}",
                    error_message, exit_code, stderr_content
                ),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
    ctx.pb.inc(1);
}
