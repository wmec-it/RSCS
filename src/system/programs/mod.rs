use colored_text:: Colorize;

use crate::conf::enums: :{DelimiterType, MessageType};
use crate::conf::vars: :{INSTALL_PROGRAMS, MAIN_THEME};
use crate::system;
use crate:: utils::message;
use crate:: utils::progress: :{create_shared_progress_bar, SharedProgressBar};

pub mod winget;

/// Installs programs with a sticky progress bar
pub fn install_programs() {
    let total_programs = INSTALL_PROGRAMS.len();
    let progress_bar = create_shared_progress_bar(total_programs, "Installing Programs");
    
    // Initialize the progress bar at the top
    progress_bar.lock().unwrap().init();
    
    for (index, program) in INSTALL_PROGRAMS.iter().enumerate() {
        system::programs::winget::winget_install(program);
        
        // Update progress after each program
        progress_bar. lock().unwrap().set_progress(index + 1);
    }

    // Mark as complete
    progress_bar.lock().unwrap().finish();

    println!("{}", "|".hex(MAIN_THEME. success));

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Finished Installing Programs Successfully! ".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

/// Installs programs with an externally managed progress bar (for full install)
pub fn install_programs_with_progress(progress_bar: &SharedProgressBar, offset: usize) {
    for (index, program) in INSTALL_PROGRAMS.iter().enumerate() {
        system::programs::winget::winget_install(program);
        
        // Update progress (offset is for when this is part of a larger operation)
        progress_bar.lock().unwrap().set_progress(offset + index + 1);
    }
}
