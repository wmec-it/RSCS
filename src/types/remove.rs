use crate::{
    AppContext,
    conf::{
        enums::{DelimiterType, MessageType},
        vars::INSTALL_PROGRAMS,
    },
    system,
    utils::message,
};

pub fn installed_programs(ctx: &mut AppContext) {
    for program in INSTALL_PROGRAMS {
        system::programs::winget::winget_remove(ctx, program);
    }

    message::success(
        ctx,
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1,
            "Finished Removing Installed Programs Successfully!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

#[allow(unused)]
pub fn unnecessary_programs(ctx: &mut AppContext) {
    // Nothing for now...
}
