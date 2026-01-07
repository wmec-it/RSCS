use crate::conf::enums::{DelimiterType, MessageType};
use crate::conf::vars::INSTALL_PROGRAMS;
use crate::system;
use crate::utils::errors;
use crate::utils::message;

pub fn install_type(install_type: &str) {
    match install_type {
        "Full Install" => run_install_full(),
        "Install Programs" => run_install_programs(),
        "Remove Installed Programs (from this script)" => remove_installed_programs(),
        "Remove Unecessary Programs (or bad ones)" => run_remove_unnecessary_programs(),
        "Skip all tweaks" => skip_all_tweaks(),
        _ => errors::idk(),
    }
}

pub fn run_install_full() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Full Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    system::programs::install_programs();
    message::seperator();
    system::tweaks::handles::run_tweaks();

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Full Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

fn run_install_programs() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Programs Install...\n|".to_string(),
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
            "Programs Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

pub fn remove_installed_programs() {
    for program in INSTALL_PROGRAMS {
        system::programs::winget::winget_remove(program);
    }

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            "Finished Removing Installed Programs Successfully!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

fn run_remove_unnecessary_programs() {
    // Nothing for now...
}

fn skip_all_tweaks() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Full Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Full Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}
