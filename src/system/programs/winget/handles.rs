use crate::{
    conf::{
        enums::{DelimiterType, MessageType},
        vars::INSTALL_PROGRAMS,
    },
    system::programs::winget,
    utils::message,
};

pub fn remove_installed_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_remove(program);
    }

    message::success(
        MessageType::Print,
        "--> Finished Removing Installed Programs Successfully!\n",
    );
}
