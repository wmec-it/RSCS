use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub mod powershell;

pub fn run_tweaks() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Running Tweaks...\n|".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    powershell::ps7::full();
    powershell::ps7telemetry::disable();
    powershell::rclick_end_task::enable();
    powershell::prefer_ipv4::enable();
    powershell::bingsearch_startmenu::disable();
}
