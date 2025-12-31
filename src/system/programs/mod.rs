use colored_text::Colorize;

use crate::conf::enums::{DelimiterType, MessageType};
use crate::conf::vars::{INSTALL_PROGRAMS, MAIN_THEME};
use crate::system;
use crate::utils::message;

pub mod winget;

pub fn install_programs() {
    for program in INSTALL_PROGRAMS {
        system::programs::winget::winget_install(program);
    }

    println!("{}", "|".hex(MAIN_THEME.success));

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Finished Installing Programs Successfully!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}
