use crate::{
    conf::{
        enums::{DelimiterType, MessageType},
        vars::INSTALL_PROGRAMS,
    },
    system,
    utils::message,
};

pub fn installed_programs() {
    for program in INSTALL_PROGRAMS {
        system::programs::winget::winget_remove(program);
    }

    message::success(
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

pub fn unnecessary_programs() {
    // Nothing for now...
}
