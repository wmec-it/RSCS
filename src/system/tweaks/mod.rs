use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub mod power;
pub mod powershell;
pub mod registry;

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
    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Running Powershell tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    powershell::ps7::full();
    powershell::ps7telemetry::disable();

    // Registry Tweaks
    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Running Registry tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    registry::rclick_end_task::enable();
    registry::prefer_ipv4::enable();
    registry::bingsearch_startmenu::disable();
    registry::darkmode::enable();
    registry::explorerpatcher_config::enable();
    registry::taskview_button::disable();

    // Power Tweaks
    message::info(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Info,
            "Running Power tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    power::ultimate_powerplan::enable();
}
