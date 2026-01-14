use crate::{
    AppContext,
    conf::enums::{DelimiterType, MessageType},
    system::{tweaks, utils},
    utils::message,
};

pub fn run_tweaks(ctx: &mut AppContext) {
    message::success(
        ctx,
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
        ctx,
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

    tweaks::powershell::ps7::full(ctx);
    tweaks::powershell::ps7telemetry::disable(ctx);

    // Registry Tweaks
    message::info(
        ctx,
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

    tweaks::registry::rclick_end_task::enable(ctx);
    tweaks::registry::prefer_ipv4::enable(ctx);
    tweaks::registry::bingsearch_startmenu::disable(ctx);
    tweaks::registry::darkmode::enable(ctx);
    tweaks::registry::explorerpatcher_config::enable(ctx);
    tweaks::registry::taskbar_taskview_button::disable(ctx);
    tweaks::registry::stickykeys_startup::disable(ctx);
    tweaks::registry::taskbar_widgets_button::disable(ctx);
    tweaks::registry::verbose_logon_messages::enable(ctx);
    tweaks::registry::hiddenfilesvisibility::enable(ctx);
    tweaks::registry::fileextensionvisibility::enable(ctx);
    tweaks::registry::detailedbsod::enable(ctx);
    tweaks::registry::explorer_homegallery::disable(ctx);
    tweaks::registry::onedrive::disable(ctx);
    tweaks::registry::displayperformance_mode::enable(ctx);
    tweaks::registry::microsoftcopilot::disable(ctx);
    tweaks::registry::taskbar_search_button::enable(ctx);
    tweaks::registry::taskbar_alignment::left(ctx);
    tweaks::registry::notificationtray::disable(ctx);
    tweaks::registry::intel_mm_lms::disable(ctx);

    utils::explorer::restart(ctx);

    // Power Tweaks
    message::info(
        ctx,
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

    tweaks::power::ultimate_powerplan::enable(ctx);
}
