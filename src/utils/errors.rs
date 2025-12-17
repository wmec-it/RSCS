use crate::conf::enums::{DelimiterType, MessageType};
use crate::utils::message;

pub fn idk() -> () {
    message::error(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Error,
            "Invalid Entry (idk what is wrong)".to_string(),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );
}
