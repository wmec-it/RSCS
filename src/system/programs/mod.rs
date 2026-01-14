use colored_text::Colorize;

use crate::conf::enums::{DelimiterType, MessageType};
use crate::conf::vars::{INSTALL_PROGRAMS, MAIN_THEME};
use crate::utils::message;
use crate::{AppContext, system};

pub mod winget;

pub fn install_programs(ctx: &mut AppContext) {
    for program in INSTALL_PROGRAMS {
        system::programs::winget::winget_install(ctx, program);
        ctx.pb.inc(1);
    }

    ctx.pb.println("|".hex(MAIN_THEME.success));

    message::success(
        ctx,
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
