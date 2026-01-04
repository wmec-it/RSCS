use crate::{
    conf::enums::{DelimiterType, MessageType},
    system::{tweaks, utils},
    utils::message,
};

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

    tweaks::powershell::ps7::full();
    tweaks::powershell::ps7telemetry::disable();

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

    tweaks::registry::rclick_end_task::enable();
    tweaks::registry::prefer_ipv4::enable();
    tweaks::registry::bingsearch_startmenu::disable();
    tweaks::registry::darkmode::enable();
    tweaks::registry::explorerpatcher_config::enable();
    tweaks::registry::taskview_button::disable();
    tweaks::registry::stickykeys_startup::disable();
    tweaks::registry::taskbarwidgets_button::disable();
    tweaks::registry::verbose_logon_messages::enable();
    tweaks::registry::hiddenfilesvisibility::enable();
    tweaks::registry::fileextensionvisibility::enable();
    tweaks::registry::detailedbsod::enable();
    tweaks::registry::explorer_homegallery::disable();
    tweaks::registry::onedrive::disable();
    tweaks::registry::displayperformance_mode::enable();
    tweaks::registry::microsoftcopilot::disable();

    utils::explorer::restart();

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

    tweaks::power::ultimate_powerplan::enable();
}
