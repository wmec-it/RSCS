use crate::{
    conf::enums::{DelimiterType, MessageType},
    utils::message,
};

pub fn all_tweaks() {
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
