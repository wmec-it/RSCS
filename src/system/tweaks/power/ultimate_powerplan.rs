// https://winutil.christitus.com/dev/tweaks/performance-plans/addultperf/

use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};
use std::process::Command;

#[allow(dead_code)]
pub fn enable() {
    let run_message: String = "Activating Ultimate power plan...".to_string();
    let success_message: String = "Successfully activated Ultimate power plan!".to_string();
    let error_message: String = "Failed to activate Ultimate power plan....".to_string();
    let command: &str = "
    try {
        $ultimatePlan = powercfg -list | Select-String -Pattern \"Ultimate Performance\"
        $ultimatePlanGUID = (powercfg -list | Select-String -Pattern \"Ultimate Performance\").Line.Split()[3]
        powercfg -setactive $ultimatePlanGUID
        Write-Host \"Ultimate Performance plan is now active.\"
    } catch {
        Write-Warning $psitem.Exception.Message
    }";

    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            run_message,
            Some(true),
            None,
            None,
        )
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
}

#[allow(dead_code)]
pub fn disable() {
    let run_message: String = "Disabling Ultimate power plan...".to_string();
    let success_message: String = "Successfully disabled Ultimate power plan!".to_string();
    let error_message: String = "Failed to disable Ultimate power plan...".to_string();
    let command: &str = "
    try {
        $ultimatePlan = powercfg -list | Select-String -Pattern \"Ultimate Performance\"
            if ($ultimatePlan) {
                $ultimatePlanGUID = $ultimatePlan.Line.Split()[3]
                $balancedPlanGUID = (powercfg -list | Select-String -Pattern \"Balanced\").Line.Split()[3]
                powercfg -setactive $balancedPlanGUID
                powercfg -delete $ultimatePlanGUID
                Write-Host \"Ultimate Performance plan has been uninstalled.\"
                Write-Host \"> Balanced plan is now active.\"
            } else {
                Write-Host \"Ultimate Performance plan is not installed.\"
            }
    } catch {
        Write-Warning $psitem.Exception.Message
    }";

    message::normal(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            run_message,
            Some(true),
            None,
            None,
        )
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
}