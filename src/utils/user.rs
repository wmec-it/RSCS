use crate::conf::enums::{DelimiterType, MessageType};
use crate::utils::message::{add_delimiter, error, info};
use rscs::core::user::sudo::enable;

pub fn enable_sudo() {
    let result = enable();

    if !result.status.success() {
        error(
            MessageType::Print,
            add_delimiter(
                DelimiterType::Layer1Error,
                String::from_utf8_lossy(&result.stdout).to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    } else {
        info(
            MessageType::Print,
            add_delimiter(
                DelimiterType::Layer1Info,
                String::from_utf8_lossy(&result.stdout).to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );
    }
}
