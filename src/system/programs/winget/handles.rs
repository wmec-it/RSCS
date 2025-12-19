use crate::{
    conf::{
        enums::{
            MessageType,
            DelimiterType
        },
        vars::INSTALL_PROGRAMS
    },
    utils::message,
    system::programs::winget
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