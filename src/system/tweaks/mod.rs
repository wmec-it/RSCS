use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub mod powershell;
pub mod registry;
pub mod power;

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

    // Powershell Tweaks
    powershell::ps7::full();
    powershell::ps7telemetry::disable();

    // Registry Tweaks
    registry::rclick_end_task::enable();
    registry::prefer_ipv4::enable();
    registry::bingsearch_startmenu::disable();
    registry::darkmode::enable();
    registry::explorerpatcher_config::enable();

    // Power Tweaks
    power::ultimate_powerplan::enable();
}
