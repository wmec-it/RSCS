use crate: :{
    conf:: enums::{DelimiterType, MessageType},
    system: :{tweaks, utils},
    utils::message,
    utils::progress::SharedProgressBar,
};

pub fn run_tweaks() {
    // Original implementation without progress bar
    run_tweaks_internal(None, 0);
}

pub fn run_tweaks_with_progress(progress_bar:  &SharedProgressBar, offset: usize) {
    run_tweaks_internal(Some(progress_bar), offset);
}

fn run_tweaks_internal(progress_bar: Option<&SharedProgressBar>, offset: usize) {
    let mut current_tweak = 0;
    
    // Helper macro to update progress after each tweak
    macro_rules! run_tweak {
        ($tweak: expr) => {
            $tweak;
            current_tweak += 1;
            if let Some(pb) = &progress_bar {
                pb.lock().unwrap().set_progress(offset + current_tweak);
            }
        };
    }

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
            "Running Powershell tweaks". to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    run_tweak!(tweaks::powershell::ps7:: full());
    run_tweak!(tweaks::powershell:: ps7telemetry::disable());

    // Registry Tweaks
    message::info(
        MessageType::Print,
        message:: add_delimiter(
            DelimiterType::Layer1Info,
            "Running Registry tweaks".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    run_tweak!(tweaks::registry::rclick_end_task:: enable());
    run_tweak!(tweaks::registry::prefer_ipv4:: enable());
    run_tweak!(tweaks::registry::bingsearch_startmenu::disable());
    run_tweak!(tweaks::registry::darkmode::enable());
    run_tweak!(tweaks::registry::explorerpatcher_config::enable());
    run_tweak!(tweaks::registry:: taskbar_taskview_button::disable());
    run_tweak!(tweaks::registry::stickykeys_startup::disable());
    run_tweak!(tweaks::registry:: taskbar_widgets_button::disable());
    run_tweak!(tweaks::registry:: verbose_logon_messages::enable());
    run_tweak!(tweaks::registry::hiddenfilesvisibility::enable());
    run_tweak!(tweaks::registry::fileextensionvisibility::enable());
    run_tweak!(tweaks::registry::detailedbsod::enable());
    run_tweak!(tweaks::registry:: explorer_homegallery::disable());
    run_tweak!(tweaks::registry::onedrive:: disable());
    run_tweak!(tweaks::registry::displayperformance_mode::enable());
    run_tweak!(tweaks::registry::microsoftcopilot:: disable());
    run_tweak!(tweaks::registry::taskbar_search_button::enable());
    run_tweak!(tweaks::registry::taskbar_alignment::left());
    run_tweak!(tweaks::registry::notificationtray:: disable());
    run_tweak!(tweaks::registry::intel_mm_lms::disable());

    utils::explorer::restart();

    // Power Tweaks
    message:: info(
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

    run_tweak!(tweaks:: power::ultimate_powerplan::enable());
}
