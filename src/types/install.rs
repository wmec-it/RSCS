use crate::{
    conf::enums::{DelimiterType, MessageType},
    system,
    utils::message,
};

pub fn full() {
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

    // TODO: Finish this shit
    // message::seperator();
    // system::programs::debloat::start();

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

pub fn programs() {
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
