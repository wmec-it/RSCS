use crate: :{
    conf:: enums::{DelimiterType, MessageType},
    conf::vars:: INSTALL_PROGRAMS,
    system,
    utils::message,
    utils::progress:: create_shared_progress_bar,
};

// Count of tweaks run in run_tweaks() - update this if you add/remove tweaks
const TWEAK_COUNT: usize = 22;

pub fn full() {
    let total_operations = INSTALL_PROGRAMS.len() + TWEAK_COUNT;
    let progress_bar = create_shared_progress_bar(total_operations, "Full Install Progress");
    
    // Initialize the progress bar
    progress_bar. lock().unwrap().init();
    
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType:: Layer1Add,
            "Starting Full Install.. .\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    // Install programs with shared progress bar
    system::programs::install_programs_with_progress(&progress_bar, 0);
    
    message::seperator();
    
    // Run tweaks with shared progress bar
    system::tweaks::handles:: run_tweaks_with_progress(&progress_bar, INSTALL_PROGRAMS.len());

    // Mark progress as complete
    progress_bar.lock().unwrap().finish();

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType:: Layer1Success,
            "Full Install Finished Successfully!!! ".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

pub fn programs() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Programs Install...\n|". to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    system::programs::install_programs();

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer2Success,
            "Programs Install Finished Successfully!!! ".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}
