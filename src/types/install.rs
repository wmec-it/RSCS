use crate::{
    AppContext,
    conf::enums::{DelimiterType, MessageType},
    system,
    utils::message,
};

pub fn full(ctx: &mut AppContext) {
    message::success(
        ctx,
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Full Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    system::programs::install_programs(ctx);
    message::seperator(ctx);
    system::tweaks::handles::run_tweaks(ctx);

    message::success(
        ctx,
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Full Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

pub fn programs(ctx: &mut AppContext) {
    message::success(
        ctx,
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Programs Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    system::programs::install_programs(ctx);

    message::success(
        ctx,
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer2Success,
            "Programs Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}
