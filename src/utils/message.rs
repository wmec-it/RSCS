use crate::conf::{enums::DelimiterType, enums::MessageType, vars::DELIMITERS, vars::MAIN_THEME};
use colored_text::Colorize;

#[allow(dead_code)]
pub fn normal(normal_type: MessageType, value: &str) -> Option<String> {
    match normal_type {
        MessageType::Return => Some(value.to_string()),
        MessageType::Print => {
            println!("{}", value);
            None
        }
    }
}

#[allow(dead_code)]
pub fn error(error_type: MessageType, value: &str) -> Option<String> {
    match error_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.error).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.error));
            None
        }
    }
}

#[allow(dead_code)]
pub fn success(success_type: MessageType, value: &str) -> Option<String> {
    match success_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.success).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.success));
            None
        }
    }
}

#[allow(dead_code)]
pub fn warning(warning_type: MessageType, value: &str) -> Option<String> {
    match warning_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.warning).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.warning));
            None
        }
    }
}

#[allow(dead_code)]
pub fn info(info_type: MessageType, value: &str) -> Option<String> {
    match info_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.info).to_string()),
        MessageType::Print => {
            println!("{}", value.hex(MAIN_THEME.info));
            None
        }
    }
}

#[allow(dead_code)]
pub fn error_banner(error_banner_type: MessageType, value: &str) -> Option<String> {
    let text_color: &str = "000000";
    match error_banner_type {
        MessageType::Return => Some(
            value
                .on_hex(MAIN_THEME.error)
                .hex(&text_color)
                .bold()
                .to_string(),
        ),
        MessageType::Print => {
            println!("{}", value.on_hex(MAIN_THEME.error).hex(&text_color).bold());
            None
        }
    }
}

#[allow(dead_code)]
pub fn add_delimiter(
    delimiter_type: DelimiterType,
    value: String,
    is_inside_section: Option<bool>,
    is_first_item_inside_section: Option<bool>,
    is_last_item_inside_section: Option<bool>,
) -> Option<String> {
    let delimiter = match delimiter_type {
        DelimiterType::Layer1 => DELIMITERS.layer1,
        DelimiterType::Layer1Info => DELIMITERS.layer1info,
        DelimiterType::Layer1Error => DELIMITERS.layer1error,
        DelimiterType::Layer1Success => DELIMITERS.layer1success,
        DelimiterType::Layer1Add => DELIMITERS.layer1add,
        DelimiterType::Layer2 => DELIMITERS.layer2,
        DelimiterType::Layer2Info => DELIMITERS.layer2info,
        DelimiterType::Layer2Error => DELIMITERS.layer2error,
        DelimiterType::Layer2Success => DELIMITERS.layer2success,
        DelimiterType::Layer2Add => DELIMITERS.layer2add,
        DelimiterType::Layer3 => DELIMITERS.layer3,
        DelimiterType::Layer3Info => DELIMITERS.layer3info,
        DelimiterType::Layer3Error => DELIMITERS.layer3error,
        DelimiterType::Layer3Success => DELIMITERS.layer3success,
        DelimiterType::Layer3Add => DELIMITERS.layer3add,
        DelimiterType::Frown => DELIMITERS.frown,
    };

    let mut result = value;

    result = format!("{} {}", delimiter, result);

    if is_inside_section.unwrap_or(false) {
        result = format!("{}", result);
    }

    if is_first_item_inside_section.unwrap_or(false) {
        result = format!("\n{}", result);
    }

    if is_last_item_inside_section.unwrap_or(false) {
        result.push('\n');
    }

    Some(result)
}

pub fn seperator() {
    println!(
        "{}",
        "-----------------------------------------------------------------------------------"
            .hex("5C5C5C")
    );
}
