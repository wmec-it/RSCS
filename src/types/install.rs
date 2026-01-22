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
    // DONT DELETE: message::seperator();
    // DONT DELETE: system::programs::debloat::start();

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

#[cfg(feature = "bambulabs")]
pub fn bambu() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Bambu Labs Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    system::programs::winget::winget_install("Bambulab.Bambustudio");

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer2Success,
            "Bambu Labs Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}
