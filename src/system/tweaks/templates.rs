use std:: process::Command;
use std::fs;
use std::env;

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub fn default(run_message: &str, success_message: &str, error_message: &str, command: &str) {
    let run_message: String = run_message.to_string();
    let success_message: String = success_message. to_string();
    let error_message: String = error_message.to_string();
    let command: &str = command;

    message:: normal(
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
        message:: success(
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
            MessageType:: Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "{}\nExit Code:  {:?}\n{}",
                    error_message,
                    output.status. code(),
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
}

pub fn admin(run_message: &str, success_message: &str, error_message:  &str, command:  &str) {
    let run_message: String = run_message.to_string();
    let success_message: String = success_message.to_string();
    let error_message:  String = error_message.to_string();

    message::normal(
        MessageType:: Print,
        message::add_delimiter(DelimiterType:: Layer1, run_message, Some(true), None, None)
            .unwrap()
            .as_str(),
    );

    // Create temp files for capturing output
    let temp_dir = env:: temp_dir();
    let stdout_file = temp_dir.join("rscs_admin_stdout.txt");
    let stderr_file = temp_dir.join("rscs_admin_stderr.txt");
    let exitcode_file = temp_dir.join("rscs_admin_exitcode.txt");

    // Escape single quotes in the command for PowerShell
    let escaped_command = command.replace("'", "''");

    // Build the elevated command that captures output to temp files
    let elevated_script = format!(
        r#"
        $stdout = '{}'
        $stderr = '{}'
        $exitcode = '{}'
        $proc = Start-Process -FilePath 'powershell' -ArgumentList '-Command', '{}; exit $LASTEXITCODE' -WindowStyle Hidden -Wait -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
        $proc.ExitCode | Out-File -FilePath $exitcode -Encoding ASCII
        "#,
        stdout_file.display(),
        stderr_file.display(),
        exitcode_file.display(),
        escaped_command
    );

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process -FilePath 'powershell' -ArgumentList '-ExecutionPolicy', 'Bypass', '-Command', '{}' -Verb RunAs -Wait -WindowStyle Hidden",
            elevated_script. replace("'", "''")
        ))
        .output()
        .expect("Failed to run PowerShell");

    // Read the captured output
    let stderr_content = fs:: read_to_string(&stderr_file).unwrap_or_default();
    let exit_code:  i32 = fs:: read_to_string(&exitcode_file)
        .unwrap_or_else(|_| "-1".to_string())
        .trim()
        .parse()
        .unwrap_or(-1);

    // Clean up temp files
    let _ = fs::remove_file(&stdout_file);
    let _ = fs::remove_file(&stderr_file);
    let _ = fs::remove_file(&exitcode_file);

    if output.status.success() && exit_code == 0 {
        message::success(
            MessageType::Print,
            message:: add_delimiter(
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
        message:: error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer2Error,
                format!(
                    "{}\nExit Code: {}\n{}",
                    error_message,
                    exit_code,
                    stderr_content
                ),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
}
