use crate::conf::enums::{DelimiterType, MessageType};
use crate::utils::message;

// TODO: Add more error types

#[allow(dead_code)]
pub fn idk() -> () {
    message::error(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Frown,
            "IDK what is wrong, something just isn't working... :(".to_string(),
            None,
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );
}

#[allow(dead_code)]
pub fn function(error: &str) -> () {
    message::error(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Error,
            error.to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

#[allow(dead_code)]
pub fn return_formatted(error: &str) -> std::io::Error {
    return std::io::Error::new(
        std::io::ErrorKind::Other,
        message::error(
            MessageType::Return,
            message::add_delimiter(
                DelimiterType::Layer1Error,
                error.to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        )
        .unwrap(),
    );
}
