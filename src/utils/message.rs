use crate::{
    AppContext,
    conf::{
        enums::{DelimiterType, MessageType},
        vars::{DELIMITERS, MAIN_THEME},
    },
};
use colored_text::Colorize;

#[allow(dead_code)]
pub fn normal(ctx: &mut AppContext, normal_type: MessageType, value: &str) -> Option<String> {
    match normal_type {
        MessageType::Return => Some(value.to_string()),
        MessageType::Print => {
            ctx.pb.println(value);
            None
        }
    }
}

#[allow(dead_code)]
pub fn error(ctx: &mut AppContext, error_type: MessageType, value: &str) -> Option<String> {
    match error_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.error).to_string()),
        MessageType::Print => {
            ctx.pb.println(value.hex(MAIN_THEME.error));
            None
        }
    }
}

#[allow(dead_code)]
pub fn success(ctx: &mut AppContext, success_type: MessageType, value: &str) -> Option<String> {
    match success_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.success).to_string()),
        MessageType::Print => {
            ctx.pb.println(value.hex(MAIN_THEME.success));
            None
        }
    }
}

#[allow(dead_code)]
pub fn warning(ctx: &mut AppContext, warning_type: MessageType, value: &str) -> Option<String> {
    match warning_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.warning).to_string()),
        MessageType::Print => {
            ctx.pb.println(value.hex(MAIN_THEME.warning));
            None
        }
    }
}

#[allow(dead_code)]
pub fn info(ctx: &mut AppContext, info_type: MessageType, value: &str) -> Option<String> {
    match info_type {
        MessageType::Return => Some(value.hex(MAIN_THEME.info).to_string()),
        MessageType::Print => {
            ctx.pb.println(value.hex(MAIN_THEME.info));
            None
        }
    }
}

#[allow(dead_code)]
pub fn error_banner(
    ctx: &mut AppContext,
    error_banner_type: MessageType,
    value: &str,
) -> Option<String> {
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
            ctx.pb
                .println(value.on_hex(MAIN_THEME.error).hex(&text_color).bold());
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
    let delimiters_enabled: bool = true;

    let delimiter = match delimiter_type {
        DelimiterType::Layer1 => {
            if delimiters_enabled {
                DELIMITERS.layer1
            } else {
                ""
            }
        }
        DelimiterType::Layer1Info => {
            if delimiters_enabled {
                DELIMITERS.layer1info
            } else {
                ""
            }
        }
        DelimiterType::Layer1Error => {
            if delimiters_enabled {
                DELIMITERS.layer1error
            } else {
                ""
            }
        }
        DelimiterType::Layer1Success => {
            if delimiters_enabled {
                DELIMITERS.layer1success
            } else {
                ""
            }
        }
        DelimiterType::Layer1Add => {
            if delimiters_enabled {
                DELIMITERS.layer1add
            } else {
                ""
            }
        }
        DelimiterType::Layer2 => {
            if delimiters_enabled {
                DELIMITERS.layer2
            } else {
                "   "
            }
        }
        DelimiterType::Layer2Info => {
            if delimiters_enabled {
                DELIMITERS.layer2info
            } else {
                "   "
            }
        }
        DelimiterType::Layer2Error => {
            if delimiters_enabled {
                DELIMITERS.layer2error
            } else {
                "   "
            }
        }
        DelimiterType::Layer2Success => {
            if delimiters_enabled {
                DELIMITERS.layer2success
            } else {
                "   "
            }
        }
        DelimiterType::Layer2Add => {
            if delimiters_enabled {
                DELIMITERS.layer2add
            } else {
                "   "
            }
        }
        DelimiterType::Layer3 => {
            if delimiters_enabled {
                DELIMITERS.layer3
            } else {
                "       "
            }
        }
        DelimiterType::Layer3Info => {
            if delimiters_enabled {
                DELIMITERS.layer3info
            } else {
                "       "
            }
        }
        DelimiterType::Layer3Error => {
            if delimiters_enabled {
                DELIMITERS.layer3error
            } else {
                "       "
            }
        }
        DelimiterType::Layer3Success => {
            if delimiters_enabled {
                DELIMITERS.layer3success
            } else {
                "       "
            }
        }
        DelimiterType::Layer3Add => {
            if delimiters_enabled {
                DELIMITERS.layer3add
            } else {
                "       "
            }
        }
        DelimiterType::Frown => {
            if delimiters_enabled {
                DELIMITERS.frown
            } else {
                ""
            }
        }
    };

    let mut result = value;

    if delimiters_enabled {
        result = format!("{} {}", delimiter, result);
    } else {
        result = format!("{}{}", delimiter, result);
    }

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

pub fn seperator(ctx: &mut AppContext) {
    ctx.pb.println(
        "-----------------------------------------------------------------------------------"
            .hex("5C5C5C"),
    );
}
