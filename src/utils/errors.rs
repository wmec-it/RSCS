use crate::conf::enums::{DelimiterType, MessageType};
use crate::utils::message;

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
