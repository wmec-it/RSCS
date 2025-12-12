use crate::conf::vars::MAIN_THEME;
use colored_text::Colorize;

#[allow(dead_code)]
pub enum MessageType {
    Return,
    Print,
}

pub fn error(error_type: MessageType, value: &str) -> Option<String> {
    match error_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.error).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.error));
            None
        }
    }
}

pub fn success(success_type: MessageType, value: &str) -> Option<String> {
    match success_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.success).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.success));
            None
        }
    }
}

pub fn warning(warning_type: MessageType, value: &str) -> Option<String> {
    match warning_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.warning).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.warning));
            None
        }
    }
}

pub fn info(info_type: MessageType, value: &str) -> Option<String> {
    match info_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.info).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.info));
            None
        }
    }
}
